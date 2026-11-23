# linode-cli nodebalancers

Beheer Linode NodeBalancers.

## Usage
- `linode-cli nodebalancers list` : toon alle NodeBalancers
- `linode-cli nodebalancers create --region regio` : maak een nieuwe NodeBalancer
- `linode-cli nodebalancers view nodebalancer_id` : toon details van een specifieke NodeBalancer
- `linode-cli nodebalancers update nodebalancer_id --label nieuw_label` : update een bestaande NodeBalancer
- `linode-cli nodebalancers delete nodebalancer_id` : verwijder een NodeBalancer
- `linode-cli nodebalancers configs list nodebalancer_id` : toon alle configuraties voor een NodeBalancer
- `linode-cli nodebalancers configs create nodebalancer_id --port poort --protocol protocol` : voeg een nieuwe configuratie toe aan een NodeBalancer
