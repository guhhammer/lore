# bun pm pkg

Управлять данными `package.json` с помощью операций `get`, `set`, `delete` и `fix`.

## Usage
- `bun pm pkg get` : получить все свойства из `package.json`
- `bun pm pkg get свойство` : получить одно свойство
- `bun pm pkg get свойство1 свойство2 свойство3 ...` : получить несколько свойств
- `bun pm pkg get свойство.атрибут` : получить вложенное свойство
- `bun pm pkg set свойство="значение"` : задать свойство
- `bun pm pkg delete свойство` : удалить свойство
- `bun pm pkg fix` : автоматически исправить распространённые ошибки в `package.json`

## Links
- More information: https://bun.com/docs/pm/cli/pm#pkg
