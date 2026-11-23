# cpufreq-info

Mostra le informazioni sulla frequenza della CPU.

## Usage
- `cpufreq-info` : mostra le informazioni sulla frequenza della CPU per tutte le CPU
- `cpufreq-info -c cpu_number` : mostra le informazioni sulla frequenza della CPU per la CPU specificata
- `cpufreq-info -l` : mostra la frequenza minima e massima consentita della CPU
- `cpufreq-info -o` : mostra la frequenza minima e massima corrente della CPU e la politica in formato tabella
- `cpufreq-info -g` : mostra le politiche di frequenza della CPU disponibili
- `cpufreq-info -f -m` : mostra la frequenza di lavoro corrente della CPU in un formato leggibile, secondo il modulo kernel cpufreq
- `sudo cpufreq-info -w -m` : mostra la frequenza di lavoro corrente della CPU in un formato leggibile, leggendola dall'hardware (disponibile solo per root)

## Links
- More information: https://manned.org/cpufreq-info
