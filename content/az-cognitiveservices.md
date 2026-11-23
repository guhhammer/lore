# az cognitiveservices

Manage Azure Cognitive Services accounts. Part of `azure-cli` (also known as `az`).

## Usage
- `az cognitiveservices account create -n account_name -g resource_group --kind api_name -l location --sku sku_name --yes` : create an API Cognitive Services account in a specific location without confirmation required
- `az cognitiveservices account list-usage -n account_name -g resource_group` : list usages for Azure Cognitive Services account
- `az cognitiveservices account deployment create -n account_name -g resource_group --deployment-name deploy_name --model-name model_name --model-version "model_version" --model-format format_name` : create a deployment for Azure Cognitive Services account
- `az cognitiveservices account commitment-plan create -n account_name -g resource_group --commitment-plan-name "plan_name" --hosting-model "hosting_model" --plan-type "plan_type" --auto-renew false|true` : create a commitment plan for Azure Cognitive Services account
- `az cognitiveservices account commitment-plan delete -g resource_group -n account_name --commitment-plan-name "plan_name"` : delete a commitment plan from Azure Cognitive Services account

## Categories
cloud, users, system

## Links
- More information: https://learn.microsoft.com/cli/azure/cognitiveservices
- tldr page: https://tldr.inbrowser.app/pages/common/az-cognitiveservices
