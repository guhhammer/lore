# cs java

Die Befehle `java` und `java-home` laden JVMs herunter und installieren sie. Der Befehl `java` führt sie auch aus.

## Usage
- `cs java -version` : zeige die Java-Version mit coursier an
- `cs java --jvm jvm_name:jvm_version -Xmx32m -Xanother_jvm_opt -jar pfad/zu/jar_name.jar` : führe eine bestimmte Java-Version mit benutzerdefinierten Eigenschaften über coursier aus
- `cs java --available` : liste alle verfügbaren JVMs im coursier-Standardindex auf
- `cs java --installed` : liste alle installierten JVMs im System mit ihrem Speicherort auf
- `cs java --jvm jvm_name:jvm_version --env` : setze eine bestimmte JVM als einmaligen Standard für die Shell-Instanz
- `eval "$(cs java --disable)"` : mache die Änderungen an den Standard-JVM-Einstellungen rückgängig
- `cs java --jvm jvm_name:jvm_version --setup` : setze eine bestimmte JVM als Standard für das gesamte System

## Links
- More information: https://get-coursier.io/docs/cli-java
