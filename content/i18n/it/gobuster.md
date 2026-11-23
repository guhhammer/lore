# gobuster

Forza brute percorsi nascosti sui web server e altro.

## Usage
- `gobuster dir -u https://example.com/ -w percorso/del/file` : scopre directory e file che corrispondono nella wordlist
- `gobuster dns -do example.com -w percorso/del/file` : scopre sottodomini
- `gobuster s3 -w percorso/del/file` : scopre bucket Amazon S3
- `gobuster vhost -u https://example.com/ -w percorso/del/file` : scopre altri virtual host sul server
- `gobuster fuzz -u https://example.com/?parameter=FUZZ -w percorso/del/file` : fuzz il valore di un parametro
- `gobuster fuzz -u https://example.com/?FUZZ=value -w percorso/del/file` : fuzz il nome di un parametro

## Links
- More information: https://github.com/OJ/gobuster#modes
