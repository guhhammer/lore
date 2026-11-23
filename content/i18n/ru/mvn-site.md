# mvn site

Генерировать веб-сайт проекта на основе информации из файла `pom.xml`.

## Usage
- `mvn site` : сгенерировать сайт проекта с помощью плагина site
- `mvn site -pl имя_модуля` : сгенерировать сайт для конкретного проекта Maven (многомодульная сборка)
- `mvn clean site` : очистить предыдущий сайт перед генерацией нового
- `mvn site -D skipTests` : пропустить тесты при генерации сайта
- `mvn site-deploy` : сгенерировать и развернуть сайт на удалённом сервере

## Links
- More information: https://maven.apache.org/plugins/maven-site-plugin/usage.html
