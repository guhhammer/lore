# cpufreq-set

Uno strumento per modificare le impostazioni della frequenza della CPU. Il valore della frequenza deve essere compreso tra l'output del comando `cpufreq-info -l`.

## Usage
- `sudo cpufreq-set -c 1 -g userspace` : imposta la politica di frequenza della CPU del core 1 su "userspace"
- `sudo cpufreq-set -c 1 -d min_frequency` : imposta la frequenza minima corrente della CPU del core 1
- `sudo cpufreq-set -c 1 -u max_frequency` : imposta la frequenza massima corrente della CPU del core 1
- `sudo cpufreq-set -c 1 -f work_frequency` : imposta la frequenza di lavoro corrente della CPU del core 1

## Links
- More information: https://manned.org/cpufreq-set
