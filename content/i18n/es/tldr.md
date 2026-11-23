# tldr

Muestra páginas de ayuda simples para herramientas de línea de comandos del proyecto tldr-pages. Nota: las opciones `--language` y `--list` no son requeridas por la especificación del cliente, pero la mayoría de los mismos las implementan.

## Usage
- `tldr comando` : imprime la página tldr para un comando específico (pista: ¡así es como has llegado hasta aquí!)
- `tldr comando subcomando` : imprime la página tldr para un subcomando específico
- `tldr -L código_de_lenguaje comando` : imprime la página tldr para un comando en el Lenguaje dado (si está disponible, de lo contrario vuelve al inglés)
- `tldr -p android|common|freebsd|linux|osx|netbsd|openbsd|sunos|windows comando` : imprime la página tldr para un comando de una plataforma específica
- `tldr -u` : actualiza la caché local de páginas tldr
- `tldr -l` : lista todas las páginas para la plataforma actual y `common`
- `tldr -l | grep comando | column` : lista todas las páginas de subcomandos disponibles para un comando
- `tldr -l | shuf -n 1 | xargs tldr` : imprime la página tldr para un comando aleatorio
