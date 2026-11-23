# elasticsearch-saml-metadata

Генерировать метаданные поставщика услуг SAML для настройки поставщика удостоверений SAML.

## Usage
- `elasticsearch-saml-metadata --realm имя_области` : сгенерировать метаданные SAML для конкретной области и вывести их в `stdout`
- `elasticsearch-saml-metadata --realm имя_области --out путь/к/файлу.xml` : сгенерировать метаданные SAML и записать их в конкретный файл
- `elasticsearch-saml-metadata -h` : показать справку
