# gnmic subscribe

Abonneer op statusupdates van gnmic-netwerkapparaten.

## Usage
- `gnmic -a ip:poort sub --path pad` : abonneer op statusupdates van een doel onder de subtree van een specifiek pad
- `gnmic -a ip:poort sub --path pad --sample-interval 30s` : abonneer op een doel met een interval van 30 seconden (standaard is 10 seconden)
- `gnmic -a ip:poort sub --path pad --stream-mode on-change --heartbeat-interval 1m` : abonneer op een doel met een interval en alleen op updates bij verandering
- `gnmic -a ip:poort sub --path pad --mode once` : abonneer op een doel voor alleen een update
- `gnmic -a ip:poort sub --path pad -e json_ietf` : abonneer op een doel en specificeer de response codering (json_ietf)

## Links
- More information: https://gnmic.openconfig.net/cmd/subscribe/
