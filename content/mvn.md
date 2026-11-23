# mvn

Apache Maven: build and manage Java-based projects.

## Usage
- `mvn compile` : compile a project
- `mvn package` : compile and package the compiled code in its distributable format, such as a `jar`
- `mvn package -D skipTests` : compile and package, skipping unit tests
- `mvn install` : install the built package in local maven repository. (This will invoke the compile and package commands too)
- `mvn clean` : delete build artifacts from the target directory
- `mvn clean package` : do a clean and then invoke the package phase
- `mvn clean -P profile package` : clean and then package the code with a given build profile
- `mvn exec:java -D exec.mainClass="com.example.Main" -D exec.args="argument1 argument2 ..."` : run a class with a main method

## Categories
packages, development

## Links
- More information: https://manned.org/mvn
- tldr page: https://tldr.inbrowser.app/pages/common/mvn
