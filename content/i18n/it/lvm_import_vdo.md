# lvm_import_vdo

Importa un volume VDO creato dal gestore VDO in un volume logico gestito da LVM (irreversibile).

## Usage
- `lvm_import_vdo /dev/mapper/vdo_volume` : importa un volume VDO con nomi automatici per VG/LV
- `lvm_import_vdo -n vg_name/lv_name /dev/mapper/vdo_volume` : importa e imposta il nome di destinazione VG/LV
- `lvm_import_vdo --dry-run /dev/mapper/vdo_volume` : mostra cosa verrebbe fatto senza modificare nulla
- `lvm_import_vdo --no-snapshot /dev/mapper/vdo_volume` : converti sul posto senza usare uno snapshot temporaneo (meno sicuro)
- `lvm_import_vdo -v -y /dev/mapper/vdo_volume` : output dettagliato e rispondi automaticamente "sì" ai prompt
- `lvm_import_vdo --vdo-config path/to/vdo.conf /dev/mapper/vdo_volume` : usa un file di configurazione del gestore VDO durante l'importazione

## Links
- More information: https://manned.org/lvm_import_vdo
