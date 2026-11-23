# apt-key

Narzędzie do zarządzania kluczami menedżera pakietów APT dla Debiana i Ubuntu.

## Usage
- `apt-key list` : wyświetl zaufane klucze
- `apt-key add plik_z_kluczem_publicznym.asc` : dodaj klucz do magazynu zaufanych kluczy
- `apt-key del id_klucza` : usuń klucz z magazynu zaufanych kluczy
- `wget -qO - https://host.tld/nazwa_pliku.key | apt-key add -` : dodaj zdalny klucz do magazynu zaufanych kluczy
- `apt-key adv --keyserver pgp.mit.edu --recv id_klucza` : dodaj klucz z serwera kluczy na podstawie ID klucza

## Links
- More information: https://manned.org/apt-key
