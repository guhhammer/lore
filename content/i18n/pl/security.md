# security

Administruj pękami kluczy, kluczami, certyfikatami oraz framework'iem Security.

## Usage
- `security list-keychains` : wypisz wszystkie dostępne pęki kluczy
- `security delete-keychain ścieżka/do/pliku.keychain` : usuń zadany pęk kluczy
- `security create-keychain -p hasło ścieżka/do/pliku.keychain` : utwórz pęk kluczy
- `security set-identity-preference -s url|hostname|serwis -c "nazwa_własna" ścieżka/do/pliku.keychain` : ustaw certyfikat który ma być używany przy stronie internetowej lub serwisie używając nazwy własnej (ta komenda nie powiedzie się gdy więcej niż jeden certyfikat ma taką samą nazwę własną)
- `security add-certificates -k plik.keychain ścieżka/do/certyfikatu.keychain.pem` : dodaj certyfikat z pliku do pęku kluczy (Jeżeli parametr -k nie został podany, domyślny pęk kluczy zostanie wykorzystany)
- `security add-trusted-cert -k ścieżka/do/pęku_kluczy_użytkownika.keychain-db ścieżka/do/certyfikatu_ca.pem` : dodaj certyfikat CA do ustawień zaufania dla każdego użytkownika
- `security remove-trusted-cert ściieżka/do/certyfikatu_ca.pem` : usuń certyfikat CA z ustawień zaufania dla każdego użytkownika

## Links
- More information: https://keith.github.io/xcode-man-pages/security.1.html
