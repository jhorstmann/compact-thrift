// From https://github.com/antlr/grammars-v4/blob/master/thrift/Thrift.g4
// $antlr-format alignTrailingComments true, columnLimit 150, minEmptyLines 1, maxEmptyLinesToKeep 1, reflowComments false, useTab false
// $antlr-format allowShortRulesOnASingleLine false, allowShortBlocksOnASingleLine true, alignSemicolons hanging, alignColons hanging

grammar Thrift;

document
    : headers+=header* definitions+=definition* EOF
    ;

header
    : 'include' include=LITERAL # includeHeader
    | 'cpp_include' include=LITERAL # cppIncludeHeader
    | 'namespace' lang=('*' | IDENTIFIER) namespace=IDENTIFIER # namespaceHeader
    ;

definition
    : 'const' fieldType identifier=IDENTIFIER '=' constValue listSeparator? # constDefinition
    | 'typedef' fieldType identifier=IDENTIFIER # typeDefinition
    | 'enum' identifier=IDENTIFIER '{' fields+=enumField* '}' # enumDefinition
    | 'struct' identifier=IDENTIFIER '{' fields+=field* '}' # structDefinition
    | 'union' identifier=IDENTIFIER '{' fields+=field* '}' # unionDefinition
    | 'exception' IDENTIFIER '{' fields+=field* '}' # exceptionDefinition
    | 'service' identifier=IDENTIFIER ('extends' extends_=IDENTIFIER)? '{' functions+=function_* '}' # serviceDefinition
    ;

enumField
    : identifier=IDENTIFIER '=' value=integer listSeparator?
    ;

field
    : (fieldId=integer ':')? requirement=fieldRequirement? type=fieldType identifier=IDENTIFIER ('=' value=constValue)? listSeparator?
    ;

fieldRequirement
    : 'required' # fieldRequired
    | 'optional' # fieldOptional
    ;

function_
    : oneway='oneway'? functionType identifier=IDENTIFIER '(' parameters+=field* ')' throwsList? listSeparator?
    ;

functionType
    : fieldType
    | 'void'
    ;

throwsList
    : 'throws' '(' field* ')'
    ;

fieldType
    : type=baseType # builtinType
    | type=IDENTIFIER # namedType
    | 'map' cppType? '<' keyType=fieldType COMMA valueType=fieldType '>' # mapType
    | 'set' cppType? '<' elementType=fieldType '>' # setType
    | 'list' cppType? '<' elementType=fieldType '>' # listType
    ;

cppType
    : 'cpp_type' value=LITERAL
    ;

constValue
    : value=integer # integerValue
    | value=DOUBLE # doubleValue
    | value=LITERAL # literalValue
    | value=IDENTIFIER # identifierValue
    | '[' (values+=constValue listSeparator?)* ']' # constListValue
    | '{' values+=constMapEntry* '}' # constMapValue
    ;

integer
    : decValue=INTEGER
    | hexValue=HEX_INTEGER
    ;

constMapEntry
    : key=constValue ':' value=constValue listSeparator?
    ;

listSeparator
    : COMMA
    | ';'
    ;

baseType
    : 'bool' # boolType
    | 'byte' # byteType
    | 'i16' # i16Type
    | 'i32' # i32Type
    | 'i64' # i64Type
    | 'double' # doubleType
    | 'string' # stringType
    | 'binary' # binaryType
    ;

INTEGER
    : ('+' | '-')? DIGIT+
    ;

HEX_INTEGER
    : '-'? '0x' HEX_DIGIT+
    ;

DOUBLE
    : ('+' | '-')? (DIGIT+ ('.' DIGIT+)? | '.' DIGIT+) (('E' | 'e') ('+' | '-')? DIGIT+)?
    ;

LITERAL
    : '"' (ESC_SEQ | ~[\\"])* '"'
    | '\'' ( ESC_SEQ | ~[\\'])* '\''
    ;

fragment ESC_SEQ
    : '\\' [rnt"'\\]
    ;

IDENTIFIER
    : (LETTER | '_') (LETTER | DIGIT | '.' | '_')*
    ;

COMMA
    : ','
    ;

fragment LETTER
    : 'A'..'Z'
    | 'a'..'z'
    ;

fragment DIGIT
    : '0'..'9'
    ;

fragment HEX_DIGIT
    : DIGIT
    | 'A'..'F'
    | 'a'..'f'
    ;

WS
    : (' ' | '\t' | '\r' '\n' | '\n')+ -> skip
    ;

SL_COMMENT
    : ('//' | '#') (~'\n')* ('\r')? '\n' -> skip
    ;

ML_COMMENT
    : '/*' .*? '*/' -> skip
    ;