use futures_io::AsyncRead;
use futures_util::AsyncReadExt;
use std::io::ErrorKind;

fn zigzag_decode64(i: u64) -> i64 {
    (i >> 1) as i64 ^ -((i & 1) as i64)
}

pub struct AsyncThriftCollector<R> {
    reader: R,
    buffer: Vec<u8>,
}

impl<R: AsyncRead + Unpin> AsyncThriftCollector<R> {
    pub fn new(reader: R, buffer: Vec<u8>) -> Self {
        Self { reader, buffer }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn buffer(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }

    async fn read_byte(&mut self) -> Result<u8, std::io::Error> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf).await?;
        self.buffer.push(buf[0]);
        Ok(buf[0])
    }

    async fn read_unsigned(&mut self) -> Result<u64, std::io::Error> {
        let mut shift = 0_u32;
        let mut value = 0_u64;
        loop {
            let byte = self.read_byte().await?;

            // overlong sequences are not treated as an error for performance reasons
            value |= ((byte & 0x7F) as u64).wrapping_shl(shift);
            shift += 7;

            if (byte & 0x80) == 0 {
                return Ok(value);
            }
        }
    }

    async fn read_zigzag(&mut self) -> Result<i64, std::io::Error> {
        let i = self.read_unsigned().await?;
        Ok(zigzag_decode64(i))
    }

    async fn read_field_header(&mut self, last_field_id: &mut i16) -> Result<u8, std::io::Error> {
        let field_header = self.read_byte().await?;

        if field_header == 0 {
            return Ok(0);
        }

        let field_type = field_header & 0x0F;
        let field_delta = field_header >> 4;
        if field_delta != 0 {
            *last_field_id += field_delta as i16;
        } else {
            *last_field_id = self.read_zigzag().await? as i16;
        }

        Ok(field_type)
    }

    pub async fn buffer_thrift_object(&mut self) -> Result<(), std::io::Error> {
        let mut last_field_id = 0_i16;
        loop {
            let field_type = self.read_field_header(&mut last_field_id).await?;
            if field_type == 0 {
                break;
            }
            self.buffer_field(field_type, false).await?;
        }

        Ok(())
    }

    async fn buffer_field(
        &mut self,
        field_type: u8,
        inside_collection: bool,
    ) -> Result<(), std::io::Error> {
        match field_type {
            1..=2 => {
                // boolean stored inside the field header outside of collections
                if inside_collection {
                    self.read_byte().await?;
                }
            }
            3 => {
                // byte
                self.read_byte().await?;
            }
            4..=6 => {
                // integers
                // no need to zigzag decode since the value will be ignored.
                self.read_unsigned().await?;
            }
            7 => {
                // double
                for _ in 0..8 {
                    self.read_byte().await?;
                }
            }
            8 | 13 => {
                // binary or uuid
                let len = self.read_unsigned().await? as usize;
                for _ in 0..len {
                    self.read_byte().await?;
                }
            }
            9 | 10 => {
                // list | set
                let header = self.read_byte().await?;
                let field_type = header & 0x0F;
                let maybe_len = (header & 0xF0) >> 4;
                let len = if maybe_len != 0x0F {
                    // high bits set high if count and type encoded separately
                    maybe_len as usize
                } else {
                    self.read_unsigned().await? as usize
                };

                if len > 0 {
                    Box::pin(async move {
                        for _ in 0..len {
                            self.buffer_field(field_type, true).await?;
                        }
                        Ok::<(), std::io::Error>(())
                    })
                    .await?;
                }
            }
            11 => {
                // map
                let len = self.read_unsigned().await? as usize;
                if len > 0 {
                    Box::pin(async move {
                        let entry_type = self.read_byte().await?;
                        let key_type = entry_type >> 4;
                        let val_type = entry_type & 0x0F;
                        for _ in 0..len {
                            self.buffer_field(key_type, true).await?;
                            self.buffer_field(val_type, true).await?;
                        }
                        Ok::<(), std::io::Error>(())
                    })
                    .await?;
                }
            }
            12 => {
                // struct | union
                Box::pin(async move {
                    let mut last_field_id = 0_i16;
                    loop {
                        let field_type = self.read_field_header(&mut last_field_id).await?;
                        if field_type == 0 {
                            return Ok::<(), std::io::Error>(());
                        }
                        self.buffer_field(field_type, false).await?;
                    }
                })
                .await?;
            }
            _ => {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidData,
                    "invalid thrift type",
                ))
            }
        }
        Ok(())
    }
}
