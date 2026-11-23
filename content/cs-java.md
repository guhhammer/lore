# cs java

The `java` and `java-home` commands fetch and install JVMs. The `java` command runs them too.

## Usage
- `cs java -version` : display Java version by using coursier
- `cs java --jvm jvm_name:jvm_version -Xmx32m -Xanother_jvm_opt -jar path/to/jar_name.jar` : call a specific Java version with custom properties using coursier
- `cs java --available` : list all the available JVM in the coursier default index
- `cs java --installed` : list all the installed JVM in the system with his own location
- `cs java --jvm jvm_name:jvm_version --env` : set a specific JVM as one-off default for the shell instance
- `eval "$(cs java --disable)"` : revert the changes for the default JVM settings
- `cs java --jvm jvm_name:jvm_version --setup` : set a specific JVM as default for the whole system

## Categories
packages

## Links
- More information: https://get-coursier.io/docs/cli-java
- tldr page: https://tldr.inbrowser.app/pages/common/cs-java
