# sc_tracediff

Anzeige von Traceroute-Pfaden, bei denen sich der Pfad geändert hat.

## Usage
- `sc_tracediff path/to/file1.warts path/to/file2.warts` : zeige den Unterschied zwischen den traceroutes in zwei `.warts`-Dateien
- `sc_tracediff -a path/to/file1.warts path/to/file2.warts` : zeige den Unterschied zwischen den traceroutes in zwei `.warts`-Dateien, einschließlich derer, die sich nicht geändert haben
- `sc_tracediff -n path/to/file1.warts path/to/file2.warts` : zeige den Unterschied zwischen den traceroutes in zwei `.warts`-Dateien und versuche, wenn möglich, DNS-Namen und nicht IP-Adressen anzuzeigen

## Links
- More information: https://www.caida.org/catalog/software/scamper/
