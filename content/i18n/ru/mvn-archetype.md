# mvn archetype

Генерировать новый проект Maven из предопределённого шаблона (архетипа).

## Usage
- `mvn archetype:generate` : сгенерировать новый проект в интерактивном режиме
- `mvn archetype:generate --define archetypeGroupId=id_группы --define archetypeArtifactId=id_артефакта --define archetypeVersion=версия --define groupId=id_группы_проекта --define artifactId=имя_проекта --define interactiveMode=false` : сгенерировать проект в неинтерактивном режиме с конкретными параметрами архетипа и проекта

## Links
- More information: https://maven.apache.org/archetype/maven-archetype-plugin/usage.html
