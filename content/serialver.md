# serialver

Return the serialVersionUID of classes. It does not set a security manager by default.

## Usage
- `serialver classnames` : display the serialVersionUID of a class
- `serialver -classpath path/to/directory classname1:classname2:...` : display the serialVersionUID for a colon-separated list of classes and resources
- `serialver -Joption classnames` : use a specific option from reference page of Java application launcher to the Java Virtual Machine

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/serialver.html
- tldr page: https://tldr.inbrowser.app/pages/common/serialver
