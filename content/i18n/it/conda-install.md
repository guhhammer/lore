# conda install

Installa pacchetti in un ambiente conda esistente.

## Usage
- `conda install pacchetto1 pacchetto2 ...` : installa uno o più pacchetti nell'ambiente conda attivo
- `conda install -c conda-forge pacchetto` : installa un singolo pacchetto nell'ambiente conda attivo utilizzando il canale conda-forge
- `conda install -c conda-forge --override-channels pacchetto` : installa un singolo pacchetto nell'ambiente conda attivo utilizzando il canale conda-forge e ignorando gli altri canali
- `conda install pacchetto=versione` : installa una versione specifica di un pacchetto
- `conda install -n ambiente pacchetto` : installa un pacchetto in un ambiente specifico
- `conda install --upgrade pacchetto` : aggiorna un pacchetto nell'ambiente attuale
- `conda install -y pacchetto` : installa un pacchetto accettando le modifiche senza richiedere conferma

## Links
- More information: https://docs.conda.io/projects/conda/en/latest/commands/install.html
