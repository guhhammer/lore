# java

Java application launcher.

## Usage
- `java classname` : execute a Java `.class` file that contains a main method by using just the class name
- `java -classpath path/to/classes1:path/to/classes2:. classname` : execute a Java program and use additional third-party or user-defined classes
- `java -jar filename.jar` : execute a `.jar` program
- `java -agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=*:5005 -jar filename.jar` : execute a `.jar` program with debug waiting to connect on port 5005
- `java -help` : display help
- `java -version` : display JDK, JRE, and HotSpot versions

## Categories
files, network

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/java.html
- tldr page: https://tldr.inbrowser.app/pages/common/java
