# ssh-agent

Memulai sebuah proses SSH Agent. SSH Agent menyimpan kunci SSH yang terdekripsi di dalam memori hingga kunci tersebut dihapus atau prosesnya dimatikan.

## Usage
- `eval $(ssh-agent)` : mulai sebuah SSH Agent untuk sesi shell saat ini
- `ssh-agent -k` : matikan agen yang sedang berjalan

## Links
- More information: https://man.openbsd.org/ssh-agent
