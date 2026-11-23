# todoist

Acest program îți permite să folosești Todoist din linia de comandă.

## Usage
- `todoist a "o_sarcină"` : adaugă o sarcină
- `todoist a "o_sarcină" --priority 1 --label-ids "idul_etichetei" --project-name "numele_proiectului" --date "tmr 9am"` : adaugă o sarcină cu prioritate ridicată cu o etichetă, proiect și dată scadentă
- `todoist q '#numele_proiectului "tmr 9am" p1 o_sarcină @numele_etichetei'` : adaugă o sarcină cu prioritate ridicată cu o etichetă, proiect și dată scadentă, folosind modul rapid
- `todoist --header --color list` : enumeră toate sarcinile cu cap de tabel și culori
- `todoist l --filter p1` : enumeră toate sarcinile cu prioritate ridicată
- `todoist l --filter '(@numele_etichetei | today) & p1'` : enumeră toate sarcinile cu prioritate ridicată de astăzi care au eticheta specificată

## Links
- More information: https://github.com/sachaos/todoist#usage
