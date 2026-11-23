# lvmpersist

Gestisce le prenotazioni persistenti (PR) su dispositivi di blocco o tutti i PV in un volume group.

## Usage
- `lvmpersist start --ourkey 0x1234abcd --vg vg_name` : avvia PR su tutti i PV in un VG con chiave locale (accesso esclusivo per impostazione predefinita)
- `lvmpersist start --ourkey 0x1234abcd --access sh --vg vg_name` : avvia PR per un VG condiviso (permette più host)
- `lvmpersist stop --ourkey 0x1234abcd --vg vg_name` : ferma PR su un VG e deregistra la chiave locale
- `lvmpersist start --ourkey 0xmy_key --removekey 0xother_key --vg vg_name` : prende il controllo di un VG locale premettendo un altro host durante l'avvio PR
- `lvmpersist remove --ourkey 0xmy_key --removekey 0xother_key --vg vg_name` : rimuove la chiave di un altro host da un VG condiviso
- `lvmpersist read --vg vg_name` : mostra chiavi registrate e prenotazioni per un VG
- `lvmpersist start --ourkey 0x1234abcd --device /dev/sdX --device /dev/mapper/mpathY` : opera su dispositivi specifici invece di un VG

## Links
- More information: https://manned.org/lvmpersist
