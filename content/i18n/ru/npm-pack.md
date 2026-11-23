# npm pack

Создавать tarball из пакета.

## Usage
- `npm pack` : создать tarball из текущего пакета в текущем каталоге
- `npm pack путь/к/каталогу_пакета` : создать tarball из определённого каталога пакета
- `npm pack --dry-run` : выполнить пробный запуск для предварительного просмотра содержимого tarball без его создания
- `npm pack --ignore-scripts` : создать tarball без выполнения скриптов жизненного цикла
- `npm pack --registry https://registry.npmjs.org/` : указать пользовательский реестр для получения метаданных пакета

## Links
- More information: https://docs.npmjs.com/cli/pack/
