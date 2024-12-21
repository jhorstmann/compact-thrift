package net.jhorstmann.compactthrift

data class Document(val header: Header, val definitions: Map<String, Definition>) {
    fun visit(visitor: DocumentVisitor) {
        visitor.visitHeader(header)
        definitions.values.forEach { visitor.visitDefinition(it) }
        visitor.finish()
    }
}

data class Header(val namespaces: Map<String, String>, val includes: List<String>)

sealed class Definition {
    abstract val identifier: String
    abstract fun <T> visit(visitor: DefinitionVisitor<T>): T
}

data class ConstantDefinition(override val identifier: String, val fieldType: FieldType, val constValue: ConstValue) : Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitConstant(this)
}

data class TypeDefinition(override val identifier: String, val fieldType: FieldType) : Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitTypeDef(this)
}

data class EnumDefinition(override val identifier: String, val fields: Map<String, EnumField>) : Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitEnum(this)
}

data class EnumField(val identifier: String, val value: Int)

data class StructDefinition(override val identifier: String, val fields: Map<String, Field>) : Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitStruct(this)
    fun hasRequiredFields(): Boolean = fields.values.any { it.required() }
}

data class UnionDefinition(override val identifier: String, val fields: Map<String, Field>) : Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitUnion(this)
    fun hasRequiredFields(): Boolean = fields.values.any { it.required() }
}

data class ExceptionDefinition(override val identifier: String, val fields: Map<String, Field>): Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitException(this)
    fun hasRequiredFields(): Boolean = fields.values.any { it.required() }
}

data class ServiceDefinition(override val identifier: String, val functions: List<ServiceFunction>): Definition() {
    override fun <T> visit(visitor: DefinitionVisitor<T>): T = visitor.visitService(this)
}

data class ServiceFunction(val identifier: String)

enum class FieldReq {
    DEFAULT, OPTIONAL, REQUIRED
}

data class Field(val id: Int, val req: FieldReq, val identifier: String, val type: FieldType) {
    fun required(): Boolean = req == FieldReq.REQUIRED
    fun optional(): Boolean = req == FieldReq.OPTIONAL
}

sealed class FieldType() {
    abstract fun <T> visit(visitor: FieldTypeVisitor<T>): T
}

data class BaseType(val type: BuiltinType): FieldType() {
    override fun <T> visit(visitor: FieldTypeVisitor<T>): T = visitor.visitBaseType(this)
}

data class NamedType(val name: String) : FieldType() {
    override fun <T> visit(visitor: FieldTypeVisitor<T>): T = visitor.visitNamedType(this)
}

data class SetType(val elementType: FieldType): FieldType() {
    override fun <T> visit(visitor: FieldTypeVisitor<T>): T = visitor.visitSetType(this)
}

data class ListType(val elementType: FieldType): FieldType() {
    override fun <T> visit(visitor: FieldTypeVisitor<T>): T = visitor.visitListType(this)
}

data class MapType(val keyType: FieldType, val valueType: FieldType): FieldType() {
    override fun <T> visit(visitor: FieldTypeVisitor<T>): T = visitor.visitMapType(this)
}

enum class BuiltinType {
    BOOL, BYTE, I16, I32, I64, DOUBLE, STRING, BINARY
}

sealed class ConstValue {
    abstract fun <T> visit(visitor: ConstantValueVisitor<T>): T
}