# java

Inicializador de programas Java.

## Usage
- `java nome_da_classe` : executa um arquivo Java `.class` que contém um método main, usando o nome da classe
- `java -classpath caminho/para/classes1:caminho/para/classes2:. nome_da_classe` : executa um programa Java e usa classes adicionais de terceiros ou definidas pelo usuário
- `java -jar nome_do_arquivo.jar` : executa um programa `.jar`
- `java -agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=*:5005 -jar nome_do_arquivo.jar` : executa um programa `.jar` com o debugger aguardando conexão na porta 5005
- `java -help` : mostra ajuda
- `java -version` : exibe a versão do JDK, JRE e HotSpot

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/java.html
