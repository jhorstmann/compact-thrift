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

The runtime support for the generated rust code can be found in the [`runtime`](https://github.com/jhorstmann/compact-thrift/tree/main/runtime) folder.

## How to run

To run this code generator you will need a Java Distribution like
[Amazon Corretto](https://docs.aws.amazon.com/corretto/latest/corretto-17-ug/downloads-list.html)
and [Apache Maven](https://maven.apache.org/download.cgi) as a build tool. Once these are installed and
their `bin` folders added to the `PATH`, the definitions for the included `parquet.thrift` can be generated
by running:

```
$ cd generator && ./generate-parquet.sh
```

## Generating rust code using macros

As an alternative to the code generator, the structures correponding to thrift definitions can also be generated with a declarative macro:

```rust
 thrift! {
     /** doc */
     struct SomeStructure {
         /** doc */
         1: required i64 offset;
         2: optional i64 length;
         3: optional list<i64> numbers;
         4: optional string data;
     }
}
```
