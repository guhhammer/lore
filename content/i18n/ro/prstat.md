# prstat

Raportează statistici despre procesele active.

## Usage
- `prstat` : examinează toate procesele și raportează statistici sortate după utilizarea CPU
- `prstat -s rss` : examinează toate procesele și raportează statistici sortate după utilizarea memoriei
- `prstat -t` : raportează un rezumat al utilizării totale pentru fiecare utilizator
- `prstat -m` : raportează informații despre microstările proceselor (microstate process accounting)
- `prstat -c -n 5 -s cpu 1` : afișează o listă cu primele 5 procese consumatoare de CPU la fiecare secundă

## Links
- More information: https://www.unix.com/man-page/sunos/1m/prstat
