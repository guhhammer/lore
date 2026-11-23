# mvn generate-sources

Генерировать исходный код для проекта Maven перед основной фазой компиляции. Эта фаза выполняется после `initialize` и перед `process-sources`.

## Usage
- `mvn generate-sources` : запустить все фазы жизненного цикла до `generate-sources`
- `mvn generate-resources` : запустить следующую фазу для генерации ресурсов
- `mvn clean generate-sources` : очистить и сгенерировать исходный код заново

## Links
- More information: https://manned.org/mvn
