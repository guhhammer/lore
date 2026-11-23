# mvn dependency

Управлять зависимостями проекта Maven и анализировать их. Предоставляет цели для просмотра, разрешения и копирования зависимостей проекта.

## Usage
- `mvn dependency:add -Dgav=id_группы:id_артефакта:версия` : добавить зависимость
- `mvn dependency:tree` : показать полное дерево зависимостей, включая прямые и транзитивные зависимости
- `mvn dependency:analyze` : проанализировать зависимости и выделить неиспользуемые или необъявленные
- `mvn dependency:copy-dependencies` : скопировать все зависимости проекта (по умолчанию в `target/dependency/`)
- `mvn dependency:resolve` : разрешить и загрузить все зависимости проекта в локальный репозиторий Maven
- `mvn dependency:resolve -U` : принудительно обновить все зависимости из удалённых репозиториев

## Links
- More information: https://maven.apache.org/plugins/maven-dependency-plugin/usage.html
