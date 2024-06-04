package net.jhorstmann.compactthrift

import java.time.Instant


val RESERVED_IDENTIFIERS = setOf<String>("type")

fun rustIdentifier(identifier: String): String {
    return if (RESERVED_IDENTIFIERS.contains(identifier)) {
        "r#$identifier"
    } else {
        identifier
    }
}

fun fieldNeedsLifeTime(document: Document, fieldType: FieldType?): Boolean {
    return if (fieldType is BaseType) {
        fieldType.type in listOf(BuiltinType.BINARY, BuiltinType.STRING)
    } else if (fieldType is NamedType) {
        definitionNeedsLifeTime(document, document.definitions[fieldType.name])
    } else if (fieldType is ListType) {
        fieldNeedsLifeTime(document, fieldType.elementType)
    } else if (fieldType is SetType) {
        fieldNeedsLifeTime(document, fieldType.elementType)
    } else if (fieldType is MapType) {
        fieldNeedsLifeTime(document, fieldType.keyType) || fieldNeedsLifeTime(document, fieldType.valueType)
    } else {
        false
    }
}
fun definitionNeedsLifeTime(document: Document, definition: Definition?): Boolean {
    val fields = when (definition) {
        is StructDefinition -> definition.fields.values
        is UnionDefinition -> definition.fields.values
        else -> emptyList()
    }
    return fields.any {
        fieldNeedsLifeTime(document, it.type)
    }
}

fun lifetimeAnnotation(document: Document, definition: Definition?): String {
    return if (definition != null && definitionNeedsLifeTime(document, definition)) {
        "<'i>"
    } else {
        ""
    }
}

private fun rustConstantValue(value: ConstValue): String {
    return value.visit(RustConstValueVisitor)
}

class RustFieldTypeVisitor(val document: Document) : FieldTypeVisitor<String> {
    override fun visitNamedType(namedType: NamedType): String = "${namedType.name}${lifetimeAnnotation(document, document.definitions[namedType.name])}"

    override fun visitBaseType(baseType: BaseType): String {
        return when (baseType.type) {
            BuiltinType.BOOL -> "bool"
            BuiltinType.BYTE -> "u8"
            BuiltinType.I16 -> "i16"
            BuiltinType.I32 -> "i32"
            BuiltinType.I64 -> "i64"
            BuiltinType.DOUBLE -> "f64"
            BuiltinType.STRING -> "Cow<'i, str>"
            BuiltinType.BINARY -> "Cow<'i, [u8]>"
        }
    }

    override fun visitMapType(mapType: MapType): String =
        "HashMap<${mapType.keyType.visit(this)}, ${mapType.valueType.visit(this)}>"

    override fun visitSetType(setType: SetType): String = "HashSet<${setType.elementType.visit(this)}>"

    override fun visitListType(listType: ListType): String = "Vec<${listType.elementType.visit(this)}>"
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


class RustGenerator(val document: Document) : DocumentVisitor {
    val code: StringBuilder = java.lang.StringBuilder()

    fun generate(): String {
        document.visit(this)
        return toString()
    }

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
        code.appendln("#[allow(non_snake_case)]")
        val namespace = header.namespaces["rust"] ?: header.namespaces["*"]
        val prefix = namespace?.let { it.replace(".", "::") + "::" } ?: ""
        code.appendln("// Generated on ${Instant.now()}")
        if (namespace != null) {
            code.appendln("// namepace=$namespace")
        }
        header.includes.forEach {
            code.appendln("use crate::$prefix$it;")
        }
        code.appendln("use std::borrow::Cow;")
        code.appendln("use compact_thrift_rs::*;")
    }

    override fun visitDefinition(definition: Definition) {
        definition.visit(RustDefinitionVisitor(document, code))
    }
}

//@formatter:off
class RustDefinitionVisitor(val document: Document, val code: StringBuilder) : DefinitionVisitor<Unit> {
    fun rustType(fieldType: FieldType, fieldReq: FieldReq = FieldReq.REQUIRED): String {
        val type = fieldType.visit(RustFieldTypeVisitor(document))

        return if (fieldReq == FieldReq.REQUIRED) {
            type
        } else {
            "Option<$type>"
        }
    }


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
            #[derive(Default, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[allow(non_camel_case_types)]
            pub struct $identifier(pub i32);

            impl $identifier {${definition.fields.values.map { """
                pub const ${rustIdentifier(it.identifier)}: Self = Self(${it.value});"""
                }.joinToString("")}
                
                const __NAMES: &'static [&'static str] = &[${definition.fields.values.map { """
                     "${it.identifier}","""}.joinToString("")}
                 ];
                
                #[inline]
                pub fn value(&self) -> i32 {
                    self.0
                }
            }""".trimIndent())

        code.appendln("""
            impl From<i32> for $identifier {
                #[inline]
                fn from(value: i32) -> Self {
                    Self(value)
                }
            }""".trimIndent())

        code.appendln("""
            impl std::fmt::Debug for $identifier {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    f.write_str(Self::__NAMES.get(self.0 as usize).unwrap_or(&"__UNKNOWN"))
                }
            }""".trimIndent())

        code.appendln("""
            impl <'i> CompactThriftProtocol<'i> for $identifier {
                const FIELD_TYPE: u8 = 5; // i32
                
                #[inline]
                fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
                    self.0 = input.read_i32()?;
                    Ok(())
                }

                #[inline]
                fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
                    output.write_i32(self.0)
                }
            }

        """.trimIndent())
    }

    override fun visitStruct(definition: StructDefinition) {
        val identifier = rustIdentifier(definition.identifier)
        val lifetimeAnnotation = lifetimeAnnotation(document, definition)
        code.appendln("""
            #[derive(Default, Clone, Debug, PartialEq)]
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub struct $identifier$lifetimeAnnotation {${definition.fields.values.map { """
                pub ${rustIdentifier(it.identifier)}: ${rustType(it.type, it.req)},""" }.joinToString("")}
            }
            
            impl$lifetimeAnnotation $identifier$lifetimeAnnotation {
                #[allow(non_camel_case_types)]
                #[allow(non_snake_case)]
                pub fn new(${definition.fields.values.map { "${rustIdentifier(it.identifier)}: impl Into<${rustType(it.type, it.req)}>" }.joinToString(", ")}) -> Self {
                    Self {${definition.fields.values.map { """
                        ${rustIdentifier(it.identifier)}: ${rustIdentifier(it.identifier)}.into(),""" }.joinToString("")}
                    }
                }
            }
            
            impl <'i> CompactThriftProtocol<'i> for $identifier$lifetimeAnnotation {
                const FIELD_TYPE: u8 = 12;
                
                #[inline(never)]
                #[allow(non_snake_case)]
                fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {${definition.fields.values.filter { it.required() }.map { """
                    let mut ${rustIdentifier(it.identifier)}_set_: bool = false;""" }.joinToString("")}
                    let mut last_field_id = 0_i16;
                    loop {
                        let field_type = input.read_field_header(&mut last_field_id)?;
                        if field_type == 0 {
                            break;
                        }
                        
                        match last_field_id {${definition.fields.values.map { """
                            ${it.id} => {
                                ${if (it.required()) "${rustIdentifier(it.identifier)}_set_ = true;" else { "" }}
                                self.${rustIdentifier(it.identifier)}.fill_field(input, field_type)?;
                            }""" }.joinToString("")}
                            _ => {
                                input.skip_field(field_type)?;
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
                    ${if (definition.fields.isEmpty()) "" else "let mut last_field_id = 0_i16;"}${definition.fields.values.map { """
                    self.${rustIdentifier(it.identifier)}.write_field(output, ${it.id}, &mut last_field_id)?;""" }.joinToString("") }
                    output.write_byte(0)?;
                    Ok(())
                }
            }""".trimIndent()
        )
    }

    override fun visitUnion(definition: UnionDefinition) {
        val identifier = definition.identifier
        val lifetimeAnnotation = lifetimeAnnotation(document, definition)

        code.appendln(
            """
            #[derive(Clone, Debug, PartialEq)]
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub enum $identifier$lifetimeAnnotation {${
                definition.fields.values.map { """
                ${it.identifier}(${rustType(it.type, FieldReq.REQUIRED)}),"""
                }.joinToString("")}
            }""".trimIndent())

        code.appendln("""
            impl$lifetimeAnnotation Default for $identifier$lifetimeAnnotation {
                fn default() -> Self {
                    Self::${definition.fields.values.first().identifier}(Default::default())
                }
            }
            """.trimIndent())

        code.appendln("""
            impl <'i> CompactThriftProtocol<'i> for $identifier$lifetimeAnnotation {
                const FIELD_TYPE: u8 = 12;
                fn fill<T: CompactThriftInput<'i>>(&mut self, input: &mut T) -> Result<(), ThriftError> {
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
                    let stop = input.read_byte()?;
                    if stop != 0 {
                        return Err(ThriftError::MissingStop)
                    }
                    
                    Ok(())
                }
                
                fn write<T: CompactThriftOutput>(&self, output: &mut T) -> Result<(), ThriftError> {
                    let mut last_field_id = 0_i16;
                    match self {${definition.fields.values.map { """
                        Self::${it.identifier}(inner) => inner.write_field(output, ${it.id}, &mut last_field_id)?, """ }.joinToString("")}
                    }
                    // STOP
                    output.write_byte(0)
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
