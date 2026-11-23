# mvn install

Устанавливать сторонние зависимости Maven и собирать проект.

## Usage
- `mvn install` : скомпилировать, протестировать, упаковать и установить проект в локальный репозиторий
- `mvn install -D skipTests` : пропустить тесты во время установки
- `mvn install -U` : принудительно обновить зависимости перед установкой
- `mvn install -D maven.test.skip=true` : пропустить компиляцию и выполнение тестов

## Links
- More information: https://manned.org/mvn
