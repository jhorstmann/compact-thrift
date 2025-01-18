.DEFAULT_GOAL := parquet

JAVA := $(wildcard *.java)
THRIFT := $(wildcard *.thrift)

generator: ${JAVA}
	mvn compile

parquet: generator ${THRIFT}
	mvn exec:java -Dexec.mainClass="net.jhorstmann.compactthrift.MainKt"
