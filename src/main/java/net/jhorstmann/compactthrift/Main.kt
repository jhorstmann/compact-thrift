package net.jhorstmann.compactthrift

import org.antlr.v4.runtime.CharStream
import org.antlr.v4.runtime.CharStreams
import java.io.File

fun main() {
    val content = File("src/test/resources/parquet.thrift").readText()
//    println(content)

//    val lexer = ThriftLexer(CharStreams.fromString(content))
//    println(lexer.nextToken())
//    println(lexer.nextToken())

    val parser = newParser(content)
    val documentContext = parser.document()
    val document = mapDocument(documentContext)
    //println(document)
    val generator = RustGenerator()
    document.visit(generator)
    println(generator)
}