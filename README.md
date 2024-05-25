# Thrift IDL parser and code generator for the compact protocol

This is an alternative implementation of the official [Apache Thrift](https://github.com/apache/thrift/)
code generator, with a focus on the [compact protocol](https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md).

The initial goal of this project is to develop a more efficient rust parser for the metadata embedded in
[Apache Parquet](https://github.com/apache/parquet-format/) files.

Higher performance is achieved by the following design decisions:

 - Fewer abstractions by focusing on the compact protocol.
   - The generated code for example inlines the reading of field headers and so avoids method calls and passing of slightly larger structure like `TFieldIdentifier`.
   - The field id and field delta can be tracked inside the generated code, similar for boolean fields, making the actual protocol code much simpler.
 - The rust target avoids moving structures from optional local variables into fields of the returned struct by directly filling the struct.
   This unfortunately requires all generated structs to implement the default trait.

Even though the initial target language for the code generator is rust, the code generator is written in [Kotlin](https://kotlinlang.org/).
The reasons for this choice are:

 - Using a jvm-based language gives access to one of the most developer friendly parser generators, [ANTLR](https://www.antlr.org/). (There is a rust implementation of Antlr, but it is mostly unmaintained at the moment.)
 - Kotlins `sealed` and `data` classes are very powerful for modeling domain objects (similar to rust enums).
 - Kotlin has built-in support for string templates, which are checked at compile time.

The runtime support for the generated rust code can be found in the [`src/main/rust`](https://github.com/jhorstmann/compact-thrift/tree/main/src/main/rust) folder.

