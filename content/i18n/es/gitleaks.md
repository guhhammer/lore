# gitleaks

Detecta secretos y claves API filtradas en repositorios Git.

## Usage
- `gitleaks detect --repo-url https://github.com/usuario/repositorio.git` : escanea un repositorio remoto
- `gitleaks detect -s ruta/al/repositorio` : escanea un directorio local
- `gitleaks detect -s ruta/al/repositorio --report ruta/a/informe.json` : crea un archivo JSON con los resultados del análisis
- `gitleaks detect -s ruta/al/repositorio --config-path ruta/al/archivo_de_configuración.toml` : utiliza un archivo de reglas personalizado
- `gitleaks detect -s ruta/al/repositorio --log-opts --since=identificador_confirmación` : inicia la búsqueda a partir de una confirmación específica
- `gitleaks protect --staged` : escanea cambios no confirmados antes de una confirmación
- `gitleaks protect --staged --verbose` : muestra información detallada que indica que partes se identificaron como fugas durante el análisis

## Links
- More information: https://github.com/gitleaks/gitleaks#usage
