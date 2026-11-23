# mvn

Ferramenta para a criação e gerenciamento de projetos Java.

## Usage
- `mvn compile` : compila um projeto
- `mvn package` : cria um artefato de distribuição utilizando o formato espeficado no `pom.xml`, por exemplo o formato `jar`
- `mvn package -D skipTests` : cria um artefato de distribuição sem executar testes unitários
- `mvn install` : instala um artefato gerado em um repositório local
- `mvn clean` : apaga artefatos gerados no diretório `target`
- `mvn clean package` : executa as fases `clean` e `package` em um projeto
- `mvn clean -P perfil package` : executa as fases `clean` e `package` em um projeto utilizando um perfil
- `mvn exec:java -D exec.mainClass="nome.do.pacote.classe" -D exec.args="argument1 argument2 ..."` : executa uma classe que possua o método `main`

## Links
- More information: https://manned.org/mvn
