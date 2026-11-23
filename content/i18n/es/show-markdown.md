# show-markdown

Muestra un archivo o cadena Markdown en la consola de forma amigable usando secuencias de escape VT100 o en un navegador usando HTML. Nota: Este comando solo puede usarse a través de PowerShell.

## Usage
- `Show-Markdown -Path ruta\al\archivo` : renderiza markdown en la consola desde un archivo
- `"# Contenido Markdown" | Show-Markdown` : renderiza markdown en la consola desde una cadena
- `Show-Markdown -Path ruta\al\archivo -UseBrowser` : abre archivo Markdown en un navegador
