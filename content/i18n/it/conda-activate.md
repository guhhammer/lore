# conda activate

Attiva un ambiente conda.

## Usage
- `conda activate myenv` : attiva un ambiente esistente chiamato `myenv`
- `conda activate percorso/a/myenv` : attiva un ambiente esistente situato in un percorso personalizzato
- `conda activate --stack myenv` : sovrapponi l'ambiente `myenv` a un ambiente precedente rendendo accessibili librerie/comandi/variabili di entrambi
- `conda activate --no-stack myenv` : avvia un ambiente pulito `myenv` senza sovrapporlo, rendendo inaccessibili librerie/comandi/variabili dell'ambiente precedente
- `conda activate -h` : mostra l'aiuto
