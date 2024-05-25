package net.jhorstmann.compactthrift

import net.jhorstmann.compactthrift.ThriftParser.*
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.misc.ParseCancellationException

class SyntaxException(message: String) : RuntimeException(message)

class LocationAwareParseCancellationException(
    val line: Int,
    val charPositionInLine: Int,
    message: String?,
    cause: Throwable?
) : ParseCancellationException(message, cause)

private object ThrowingErrorListener : BaseErrorListener() {

    override fun syntaxError(
        recognizer: Recognizer<*, *>?,
        offendingSymbol: Any?,
        line: Int,
        charPositionInLine: Int,
        msg: String?,
        e: RecognitionException?
    ) {
        throw LocationAwareParseCancellationException(line, charPositionInLine, msg, e)
    }
}

private fun newParser(input: CharStream): ThriftParser {
    val lexer = ThriftLexer(input)
    lexer.removeErrorListeners()
    lexer.addErrorListener(ThrowingErrorListener)
    val parser = ThriftParser(BufferedTokenStream(lexer))
    parser.removeErrorListeners()
    parser.addErrorListener(ThrowingErrorListener)
    return parser
}

internal fun newParser(input: String): ThriftParser {
    return newParser(CharStreams.fromString(input))
}

fun mapHeader(headers: List<HeaderContext>): Header {
    val namespaces = mutableMapOf<String, String>()
    val includes = mutableListOf<String>()

    headers.forEach {
        when (it) {
            is NamespaceHeaderContext -> {
                namespaces[it.lang.text] = it.namespace.text
            }
            is IncludeHeaderContext -> {
                includes.add(it.include.text)
            }
            is CppIncludeHeaderContext -> {
                TODO("CPP include")
            }
            else -> throw RuntimeException("Unkown header type ${it.javaClass.name}")
        }
    }

    return Header(namespaces, includes)
}

fun mapDefinition(definitionCtx: DefinitionContext): Definition {
    return when (definitionCtx) {
        is ConstDefinitionContext -> {
            ConstantDefinition(
                definitionCtx.identifier.text,
                mapFieldType(definitionCtx.fieldType()),
                mapConstantValue(definitionCtx.constValue())
            )
        }
        is TypeDefinitionContext -> {
            TypeDefinition(definitionCtx.identifier.text, mapFieldType(definitionCtx.fieldType()))
        }
        is EnumDefinitionContext -> {
            EnumDefinition(definitionCtx.identifier.text, mapEnumFields(definitionCtx.fields))
        }
        is StructDefinitionContext -> {
            StructDefinition(definitionCtx.identifier.text, mapStructOrUnionFields(definitionCtx.fields))
        }
        is UnionDefinitionContext -> {
            UnionDefinition(definitionCtx.identifier.text, mapStructOrUnionFields(definitionCtx.fields))
        }
        is ExceptionDefinitionContext -> {
            TODO("ExceptionDefinition")
        }
        is ServiceDefinitionContext -> {
            TODO("ServiceDefinition")
        }
        else -> {
            throw RuntimeException("unknown definition type ${definitionCtx.javaClass.name}")
        }
    }
}

fun mapFieldType(fieldType: FieldTypeContext): FieldType {
    return when (fieldType) {
        is BuiltinTypeContext -> {
            val baseType = fieldType.type;
            val builtinType = when (baseType) {
                is BoolTypeContext -> BuiltinType.BOOL
                is ByteTypeContext -> (BuiltinType.BYTE)
                is I16TypeContext -> (BuiltinType.I16)
                is I32TypeContext -> (BuiltinType.I32)
                is I64TypeContext -> (BuiltinType.I64)
                is DoubleTypeContext -> BuiltinType.DOUBLE
                is StringTypeContext -> BuiltinType.STRING
                is BinaryTypeContext -> BuiltinType.BINARY
                else -> throw RuntimeException("Unknown base type ${baseType.javaClass.name}")
            }
            BaseType(builtinType)
        }
        is NamedTypeContext -> NamedType(fieldType.type.text)
        is MapTypeContext -> MapType(mapFieldType(fieldType.keyType), mapFieldType(fieldType.valueType))
        is ListTypeContext -> ListType(mapFieldType(fieldType.elementType))
        is SetTypeContext -> SetType(mapFieldType(fieldType.elementType))
        else -> throw RuntimeException("Unknown field type ${fieldType.javaClass.name}")
    }
}

fun mapConstantValue(constValue: ConstValueContext): ConstValue {
    TODO("ConstValue")
}

fun mapEnumFields(fields: MutableList<EnumFieldContext>): Map<String, EnumField> {
    return fields.map { EnumField(it.identifier.text, mapInteger(it.value)) }
        .associateByTo(LinkedHashMap()) { it.identifier }
}

fun mapStructOrUnionFields(fields: MutableList<FieldContext>): Map<String, Field> {
    var currentFieldId = -1
    return fields.map {
        var fieldId = it.fieldId?.let { mapInteger(it) }
        if (fieldId == null) {
            ++currentFieldId
        } else {
            currentFieldId = fieldId
        }
        Field(
            currentFieldId,
            it.requirement?.let { mapFieldReq(it) } ?: FieldReq.DEFAULT,
            it.identifier.text,
            mapFieldType(it.type))
    }.associateByTo(LinkedHashMap()) { it.identifier }
}

fun mapFieldReq(req: FieldRequirementContext): FieldReq {
    return when (req) {
        is FieldRequiredContext -> FieldReq.REQUIRED
        is FieldOptionalContext -> FieldReq.OPTIONAL
        else -> throw RuntimeException("Unknown field requirment ${req.javaClass.name}")
    }
}

fun mapInteger(i: IntegerContext): Int {
    val dec = i.decValue
    return if (dec != null) {
        dec.text.toInt(10)
    } else {
        val hex = i.hexValue.text
        val neg = hex.startsWith('-')

        // strip 0x or -0x prefix
        val start = if (neg) {
            3
        } else {
            2
        }
        val value = i.hexValue.text.substring(start).toInt(16)
        if (neg) {
            -value
        } else {
            value
        }
    }
}

fun mapDocument(docCtx: DocumentContext): Document {
    return Document(
        mapHeader(docCtx.headers),
        docCtx.definitions.map { mapDefinition(it) }.associateBy { it.identifier })
}
