# aws amplify

Piattaforma di sviluppo per la creazione di applicazioni mobile e web sicure e scalabili.

## Usage
- `aws amplify create-app --name nome_app --description descrizione --repository repo_url --platform platform --environment-variables env_vars --tags tags` : crea una nuova app Amplify
- `aws amplify delete-app --app-id app_id` : elimina un'app Amplify esistente
- `aws amplify get-app --app-id app_id` : ottieni dettagli di un'app Amplify specifica
- `aws amplify list-apps` : elenca tutte le app Amplify
- `aws amplify update-app --app-id app_id --name nuovo_nome --description nuova_description --repository nuova_repo_url --environment-variables nuove_env_vars --tags nuove_tags` : aggiorna le impostazioni di un'app Amplify
- `aws amplify create-backend-environment --app-id app_id --environment-name nome_env --deployment-artifacts artifacts` : aggiungi un nuovo ambiente backend a un'app Amplify
- `aws amplify delete-backend-environment --app-id app_id --environment-name nome_env` : rimuovi un ambiente backend da un'app Amplify
- `aws amplify list-backend-environments --app-id app_id` : elenca tutti gli ambienti backend in un'app Amplify

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/amplify/
