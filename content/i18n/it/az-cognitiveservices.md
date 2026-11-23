# az cognitiveservices

Gestisce gli account Azure Cognitive Services. Parte di `azure-cli` (noto anche come `az`).

## Usage
- `az cognitiveservices account create -n account_name -g resource_group --kind api_name -l location --sku sku_name --yes` : crea un account Cognitive Services API in una posizione specifica senza conferma richiesta
- `az cognitiveservices account list-usage -n account_name -g resource_group` : elenca gli utilizzi per l'account Azure Cognitive Services
- `az cognitiveservices account deployment create -n account_name -g resource_group --deployment-name deploy_name --model-name model_name --model-version "model_version" --model-format format_name` : crea un deployment per l'account Azure Cognitive Services
- `az cognitiveservices account commitment-plan create -n account_name -g resource_group --commitment-plan-name "plan_name" --hosting-model "hosting_model" --plan-type "plan_type" --auto-renew false|true` : crea un piano di commitment per l'account Azure Cognitive Services
- `az cognitiveservices account commitment-plan delete -g resource_group -n account_name --commitment-plan-name "plan_name"` : elimina un piano di commitment dall'account Azure Cognitive Services

## Links
- More information: https://learn.microsoft.com/cli/azure/cognitiveservices
