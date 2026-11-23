# az webapp

Gestisce le applicazioni web ospitate nei servizi cloud Azure. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az webapp list-runtimes -os windows|linux` : elenca i runtime disponibili per un'applicazione web
- `az webapp up -n nome -l posizione -r runtime` : crea un'applicazione web
- `az webapp list` : elenca tutte le applicazioni web
- `az webapp delete -n nome -g gruppo_risorse` : elimina un'applicazione web specifica

## Links
- More information: https://learn.microsoft.com/cli/azure/webapp
