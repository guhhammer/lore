# az quantum

Gestisce i workspace Azure Quantum e invia job quantistici ai provider (preview, richiede estensione quantum).

## Usage
- `az quantum workspace create -g GruppoRisorse -l Posizione -w Workspace -a NomeAccountArchiviazione` : crea un nuovo workspace Azure Quantum
- `az quantum workspace list` : elenca tutti i workspace Azure Quantum
- `az quantum workspace set -g GruppoRisorse -w Workspace` : imposta un workspace Azure Quantum predefinito
- `az quantum job submit -g GruppoRisorse -w Workspace -l Posizione -t Id --job-name Job --job-input-file QirBitcode.bc --job-input-format qir.v1` : invia un job quantistico QIR a un target
- `az quantum job list -g GruppoRisorse -l Posizione -w Workspace` : elenca tutti i job in un Quantum Workspace
- `az quantum job output -g GruppoRisorse -w Workspace --job-id Job` : ottiene l'output di un job quantistico
- `az quantum offerings list -l Posizione` : elenca le offerte dei provider disponibili in una posizione
- `az quantum target set -t Id` : imposta un target predefinito per l'invio dei job

## Links
- More information: https://learn.microsoft.com/cli/azure/quantum
