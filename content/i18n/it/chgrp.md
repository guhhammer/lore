# chgrp

Cambia il gruppo proprietario di file e directory.

## Usage
- `chgrp gruppo percorso/del/file_o_directory` : cambia il gruppo proprietario di un file/directory
- `chgrp -R gruppo percorso/della/directory` : cambia ricorsivamente il gruppo proprietario di una directory e dei suoi contenuti
- `chgrp -h gruppo percorso/del/symlink` : cambia il gruppo proprietario di un link simbolico
- `chgrp --reference percorso/del/file_riferimento percorso/del/file_o_directory` : cambia il gruppo proprietario di un file/directory rendendolo uguale a quello di un altro file di riferimento
