# aa-status

Tampilkan daftar modul AppArmor yang saat ini dimuat.

## Usage
- `sudo aa-status` : periksa status
- `sudo aa-status --json` : tampilkan status dalam format JSON
- `sudo aa-status --pretty-json` : tampilkan status dalam format JSON yang rapi (pretty)
- `sudo aa-status --profiled` : tampilkan jumlah kebijakan yang dimuat
- `sudo aa-status --enforced` : tampilkan jumlah kebijakan enforce yang dimuat
- `sudo aa-status --complaining` : tampilkan jumlah kebijakan non-enforce yang dimuat
- `sudo aa-status --kill` : tampilkan jumlah kebijakan enforce yang dimuat yang mematikan tugas

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-status.8
