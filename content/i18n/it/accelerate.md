# accelerate

Libreria che permette di eseguire lo stesso codice PyTorch su qualsiasi configurazione distribuita.

## Usage
- `accelerate env` : stampa le informazioni sull'ambiente
- `accelerate config` : crea interattivamente un file di configurazione
- `accelerate estimate-memory nome/modello` : stampa il costo stimato in memoria GPU per eseguire un modello Hugging Face con diversi tipi di dati
- `accelerate test --config_file percorso/del/config.yaml` : testa un file di configurazione Accelerate
- `accelerate launch percorso/del/script.py --cpu` : esegui un modello su CPU con Accelerate
- `accelerate launch percorso/del/script.py --multi_gpu --num_machines 2` : esegui un modello su multi-GPU con Accelerate, con 2 macchine

## Links
- More information: https://huggingface.co/docs/accelerate/index
