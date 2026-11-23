# ip stats

Gestisce e mostra le statistiche dell'interfaccia.

## Usage
- `ip st` : mostra tutte le statistiche dell'interfaccia su tutti i dispositivi di rete
- `ip st show dev interfaccia_network` : mostra le statistiche per un'interfaccia di rete specifica
- `ip st show group link` : mostra le statistiche del livello di collegamento (come `ip -statistics link show`)
- `ip st show group offload` : mostra le statistiche di offload hardware per tutti i dispositivi
- `ip st show dev interfaccia_network group offload` : mostra le statistiche di offload per un'interfaccia specifica
- `ip st show dev interfaccia_network group offload subgroup l3_stats|cpu_hit|hw_stats_info` : mostra un sottogruppo di offload specifico
- `ip st show group afstats subgroup mpls` : mostra statistiche specifiche per famiglia di indirizzi (es. MPLS)
- `ip st set dev interfaccia_network l3_stats on` : abilita la raccolta di statistiche hardware di Livello 3 su un dispositivo

## Links
- More information: https://manned.org/ip-stats
