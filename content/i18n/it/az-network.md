# az network

Gestisce le risorse di rete Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az network list-usages` : elenca le risorse di rete in una regione che utilizzano la quota di sottoscrizione
- `az network vnet list` : elenca tutte le reti virtuali in una sottoscrizione
- `az network vnet create --address-prefixes 10.0.0.0/16 -n vnet -g nome_gruppo --subnet-name sottorete --subnet-prefixes 10.0.0.0/24` : crea una rete virtuale
- `az network nic update --accelerated-networking true -n nic -g gruppo_risorse` : abilita il networking accelerato per una scheda di interfaccia di rete

## Links
- More information: https://learn.microsoft.com/cli/azure/network
