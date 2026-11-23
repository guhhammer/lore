# koji download-build

Scarica un pacchetto compilato.

## Usage
- `koji download-build BuildID|RPM|NVR` : scarica tutti gli RPM da una build specifica
- `koji download-build BuildID|RPM|NVR --key chiave` : scarica gli RPM firmati con la chiave data
- `koji download-build BuildID|RPM|NVR --arch x86_64,aarch64,noarch,...` : scarica solo gli RPM per una data architettura
- `koji download-build RPM --rpm` : scarica l'RPM specificato
- `koji download-build -h` : mostra l'aiuto

## Links
- More information: https://docs.pagure.org/koji/
