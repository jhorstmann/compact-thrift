package net.jhorstmann.compactthrift

import java.time.Instant

fun rustType(fieldType: FieldType, fieldReq: FieldReq = FieldReq.REQUIRED): String {
    val type = fieldType.visit(RustFieldTypeVisitor)

    return if (fieldReq == FieldReq.REQUIRED) {
        type
    } else {
        "Option<$type>"
    }
}

val RESERVED_IDENTIFIERS = setOf<String>("type")

fun rustIdentifier(identifier: String): String {
    return if (RESERVED_IDENTIFIERS.contains(identifier)) {
        "r#$identifier"
    } else {
        identifier
    }
}

private fun rustConstantValue(value: ConstValue): String {
    return value.visit(RustConstValueVisitor)
}

object RustFieldTypeVisitor : FieldTypeVisitor<String> {
    override fun visitNamedType(namedType: NamedType): String = namedType.name

    override fun visitBaseType(baseType: BaseType): String {
        return when (baseType.type) {
            BuiltinType.BOOL -> "bool"
            BuiltinType.BYTE -> "u8"
            BuiltinType.I16 -> "i16"
            BuiltinType.I32 -> "i32"
            BuiltinType.I64 -> "i64"
            BuiltinType.DOUBLE -> "f64"
            BuiltinType.STRING -> "String"
            BuiltinType.BINARY -> "Vec<u8>"
        }
    }

    override fun visitMapType(mapType: MapType): String =
        "HashMap<${rustType(mapType.keyType)}, ${rustType(mapType.valueType)}>"

    override fun visitSetType(setType: SetType): String = "HashSet<${rustType(setType.elementType)}>"

    override fun visitListType(listType: ListType): String = "Vec<${rustType(listType.elementType)}>"
}

object RustConstValueVisitor : ConstantValueVisitor<String> {
    override fun visitInt(value: Int): String = "$value"

    override fun visitDouble(value: Double): String = "${value}_f64"

    override fun visitLiteral(value: String): String = "\"${value.replace("\"", "\\\"")}\""

    override fun visitIdentifier(value: String): String = rustIdentifier(value)

    override fun visitList(value: List<ConstValue>): String = "&[${value.map { it.visit(this) }}]"

    override fun visitMap(value: Map<ConstValue, ConstValue>): String {
        return "&[${value.map { "(${it.key.visit(this)}, ${it.value.visit(this)}"}.joinToString(", ")}]"
    }
}


class RustGenerator() : DocumentVisitor {
    val code: StringBuilder = java.lang.StringBuilder()

    override fun toString(): String {
        return code.toString()
    }

    override fun finish() {
        code.appendln(
            """
            #[cfg(test)]
            mod tests {
                #[test]
                fn test_compile() {

                }
            }
        """.trimIndent()
        )
    }

    override fun visitHeader(header: Header) {
        val namespace = header.namespaces["rust"] ?: header.namespaces["*"]
        val prefix = namespace?.let { it.replace(".", "::") + "::" } ?: ""
        code.appendln("// Generated on ${Instant.now()}")
        if (namespace != null) {
            code.appendln("// namepace=$namespace")
        }
        header.includes.forEach {
            code.appendln("use crate::$prefix$it;")
        }
        code.appendln("use compact_thrift_rs::*;")
    }

    override fun visitDefinition(definition: Definition) {
        definition.visit(RustDefinitionVisitor(code))
    }
}

//@formatter:off
class RustDefinitionVisitor(val code: StringBuilder) : DefinitionVisitor {
    override fun visitConstant(definition: ConstantDefinition) {
        code.appendln("""
            pub const ${rustIdentifier(definition.identifier)}: ${rustType(definition.fieldType)} = ${rustConstantValue(definition.constValue)};
        """.trimIndent())
    }

    override fun visitTypeDef(definition: TypeDefinition) {
        code.appendln("""
            pub type ${rustIdentifier(definition.identifier)} = ${rustType(definition.fieldType)};
        """.trimIndent())
    }

    override fun visitEnum(definition: EnumDefinition) {
        val identifier = rustIdentifier(definition.identifier)
        code.appendln(
            """
            #[derive(Default, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[allow(non_camel_case_types)]
            pub struct $identifier(i32);

            impl $identifier {${
                definition.fields.values.map {
                    """
                pub const ${rustIdentifier(it.identifier)}: i32 = ${it.value};"""
                }.joinToString("")}
                
                pub fn value(&self) -> i32 {
                    self.0
                }
            }""")
        code.appendln("""
            impl From<i32> for $identifier {
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }""".trimIndent())

        code.appendln("""
            impl CompactThriftProtocol for $identifier {
                const FIELD_TYPE: u8 = 5; // i32
                fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
                    self.0 = input.read_i32()?;
                    Ok(())
                }

                fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
                    output.write_i32(self.0)
                }
            }

        """.trimIndent())
    }

    override fun visitStruct(definition: StructDefinition) {
        val identifier = rustIdentifier(definition.identifier)
        code.appendln("""
            #[derive(Default, Clone, Debug)]
            #[allow(non_camel_case_types)]
            pub struct $identifier {${definition.fields.values.map { """
                pub ${rustIdentifier(it.identifier)}: ${rustType(it.type, it.req)},""" }.joinToString("")}
            }
            
            impl CompactThriftProtocol for $identifier {
                const FIELD_TYPE: u8 = 12;
                
                #[inline(never)]
                fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {${definition.fields.values.filter { it.required() }.map { """
                    let mut ${rustIdentifier(it.identifier)}_set_: bool = false;""" }.joinToString("")}
                    let mut last_field_id = 0_i16;
                    loop {
                        let field_header = input.read_byte()?;
            
                        if field_header == 0 {
                            break;
                        }
            
                        let field_type = field_header & 0x0F;
                        let field_delta = field_header >> 4;
                        if field_delta != 0 {
                            last_field_id += field_delta as i16;
                        } else {
                            last_field_id = input.read_i16()?;
                        }
                        
                        match last_field_id {${definition.fields.values.map { """
                            ${it.id} => {
                                ${if (it.required()) "${rustIdentifier(it.identifier)}_set_ = true;" else { "" }}
                                self.${rustIdentifier(it.identifier)}.fill_field(input, field_type)?;
                            }""" }.joinToString("")}
                            _ => {
                                skip_field(input, field_type)?;
                            }
                        }
                    }
                    ${if (definition.hasRequiredFields()) { """
                    if ${definition.fields.values.filter { it.required() }.map { "!${rustIdentifier(it.identifier)}_set_" }.joinToString(" || ")} {
                        return Err(ThriftError::MissingField)
                    }
                    """ } else { "" }}
                    
                    Ok(())
                }
                
                fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
                    unimplemented!("${definition.identifier}::write")
                }
            }""".trimIndent()
        )
    }

    override fun visitUnion(definition: UnionDefinition) {
        val identifier = definition.identifier
        code.appendln(
            """
            #[derive(Clone, Debug)]
            #[allow(non_camel_case_types)]
            pub enum $identifier {${
                definition.fields.values.map { """
                ${it.identifier}(${rustType(it.type, FieldReq.REQUIRED)}),"""
                }.joinToString("")}
            }""".trimIndent())

        code.appendln("""
            impl Default for $identifier {
                fn default() -> Self {
                    Self::${definition.fields.values.first().identifier}(Default::default())
                }
            }
            """.trimIndent())

        code.appendln("""
            impl CompactThriftProtocol for $identifier {
                const FIELD_TYPE: u8 = 12;
                fn fill<T: CompactThriftInput>(&mut self, input: &mut T) -> Result<(), ThriftError> {
                    let field_type = input.read_byte()?;
                        
                    if field_type == 0 {
                        return Err(ThriftError::InvalidType);
                    }
                        
                    let field_delta = (field_type & 0xF0) >> 4;
                    let field_id = if field_delta != 0 {
                        field_delta as i16
                    } else {
                        input.read_i16()?
                    };

                    match field_id {${definition.fields.values.map { """
                        ${it.id} => {
                            *self = Self::${it.identifier}(Default::default());
                            #[allow(unreachable_patterns)]
                            match self {
                                Self::${it.identifier}(inner) => inner.fill(input)?,
                                _ => unsafe { std::hint::unreachable_unchecked() },
                            }
                        }""" }.joinToString("")}
                        _ => {
                            return Err(ThriftError::MissingField)
                        }
                    }
                    
                    Ok(())
                }
                
                fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
                    unimplemented!("${definition.identifier}::write")
                }
            }
        """.trimIndent())
    }

    override fun visitException(definition: ExceptionDefinition) {
        TODO("Not yet implemented")
    }

    override fun visitService(definition: ServiceDefinition) {
        TODO("Not yet implemented")
    }
}
//@formatter:on
