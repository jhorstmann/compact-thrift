package net.jhorstmann.compactthrift

interface DocumentVisitor {
    fun visitHeader(header: Header)
    fun visitDefinition(definition: Definition)
    fun finish() {

    }
}

interface DefinitionVisitor {
    fun visitConstant(definition: ConstantDefinition)
    fun visitTypeDef(definition: TypeDefinition)
    fun visitEnum(definition: EnumDefinition)
    fun visitStruct(definition: StructDefinition)
    fun visitUnion(definition: UnionDefinition)
    fun visitException(definition: ExceptionDefinition)
    fun visitService(definition: ServiceDefinition)
}

interface FieldTypeVisitor<T> {
    fun visitNamedType(namedType: NamedType): T
    fun visitBaseType(baseType: BaseType): T
    fun visitMapType(mapType: MapType): T
    fun visitSetType(setType: SetType): T
    fun visitListType(listType: ListType): T
}

interface ConstantValueVisitor<T> {
    fun visitInt(value: Int): T
    fun visitDouble(value: Double): T
    fun visitLiteral(value: String): T
    fun visitIdentifier(value: String): T
    fun visitList(value: List<ConstValue>): T
    fun visitMap(value: Map<ConstValue, ConstValue>): T
}