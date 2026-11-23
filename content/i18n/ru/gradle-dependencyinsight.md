# gradle-dependencyinsight

Отображать подробную информацию о конкретной зависимости в проекте Gradle.

## Usage
- `gradle dependencyInsight --dependency имя_пакета` : показать информацию о конкретной зависимости
- `gradle dependencyInsight --dependency имя_пакета --configuration имя_конфигурации` : показать информацию о зависимости в конкретной конфигурации
- `gradle :подпроект:dependencyInsight --dependency имя_пакета` : показать информацию для конкретного подпроекта
- `gradle dependencyInsight --dependency имя_пакета -i` : показать информацию с полным путём зависимости
