# uv publish

Загружать дистрибутивы в индекс.

## Usage
- `uv publish` : опубликовать пакеты из каталога `dist/` (поведение по умолчанию)
- `uv publish --publish-url https://upload.pypi.org/legacy/` : опубликовать в определённый репозиторий
- `uv publish -u имя_пользователя -p пароль` : опубликовать с определённым именем пользователя и паролем
- `uv publish -t ваш_api_токен` : опубликовать с использованием API-токена
- `uv publish путь/к/dist/*.whl путь/к/dist/*.tar.gz` : опубликовать определённые файлы дистрибутивов
- `uv publish --publish-url https://test.pypi.org/legacy/` : опубликовать на TestPyPI для тестирования

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-publish
