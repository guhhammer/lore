# csc

Compilatore per Microsoft C#.

## Usage
- `csc percorso/del/file_input_a.cs percorso/del/file_input_b.cs ...` : compila uno o più file C# in un eseguibile da command-line
- `csc /out:percorso/del/file percorso/del/file_input.cs` : specifica il nome del file output
- `csc /target:library percorso/del/file_input.cs` : compila in una libreria `.dll` invece che in un eseguibile
- `csc /reference:percorso/della/libreria.dll percorso/del/file_input.cs` : referenzia un altro assembly
- `csc /resource:percorso/del/file_risorsa percorso/del/file_input.cs` : includi una risorsa
- `csc /doc:percorso/della/documentazione.xml percorso/del/file_input.cs` : genera una documentazione XML automaticamente
- `csc /win32icon:percorso/dell/icona.ico percorso/del/file_input.cs` : specifica un'icona
- `csc /keyfile:percorso/della/chiave.snk percorso/del/file_input.cs` : firma un assembly con un nome sicuro utilizzando una chiave
