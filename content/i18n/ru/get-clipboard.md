# get-clipboard

Команда PowerShell для получения содержимого буфера обмена. Примечание: в качестве псевдонима для `Get-Clipboard` можно использовать `gcb`.

## Usage
- `Get-Clipboard` : получить текст из буфера обмена
- `Get-Clipboard -TextFormatType Text|Html|Rtf` : получить содержимое буфера обмена в определённом текстовом формате
- `Get-Clipboard -Raw` : получить необработанное (raw) содержимое буфера обмена
- `Get-Clipboard -Format Image` : извлечь изображение
- `Get-Clipboard -Format FileDropList` : получить список путей к файлам, скопированным в Проводнике
