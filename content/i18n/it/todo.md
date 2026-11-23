# todo

Un semplice gestore per i todo da linea di comando.

## Usage
- `todo list --startable` : elenco dei task che possono essere inizializzati
- `todo new cose_da_fare -l lavoro` : aggiungere un nuovo task alla lista delle cose da fare per lavoro
- `todo edit --location nome_località task_id` : aggiungere una località ad un task con un dato ID
- `todo show task_id` : mostrare i dettagli di un task
- `todo done task_id1 task_id2 ...` : contrassegnare un task con un ID specifico come completato
- `todo delete task_id` : eliminare un task
- `todo flush` : eliminare un task completato e ripristinare gli ID dei task rimanenti

## Links
- More information: https://todoman.readthedocs.io/en/stable/man.html
