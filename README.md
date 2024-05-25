# Thrift IDL parser and code generator for the compact protocol

This is an alternative implementation of the official [Apache Thrift](https://github.com/apache/thrift/)
code generator, with a focus on the [compact protocol](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md).

The initial goal of this project is to develop a more efficient rust parser for the metadata embedded in
[Apache Parquet](https://github.com/apache/parquet-format/) files. 

Even though the initial target language for the code generator is rust, the code generator is written in [Kotlin](https://kotlinlang.org/).
The reasons for this choice are:

 - Using a jvm-based language gives access to one of the most developer friendly parser generators, [ANTLR](https://www.antlr.org/). (There is a rust implementation of Antlr, but it is mostly unmaintained at the moment.)
 - Kotlins `sealed` and `data` classes are very powerful for modeling domain objects (similar to rust enums).
 - Kotlin has built-in support for string templates, which are checked at compile time.

The runtime support for the generated rust code can be found in the [`src/main/rust`](tree/main/src/main/rust) folder.

