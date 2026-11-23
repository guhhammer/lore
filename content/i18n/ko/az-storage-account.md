# az storage account

Azure에서 스토리지 계정을 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az storage account create -n 스토리지_계정_이름 -g azure_리소스_그룹 --location azure_위치 --sku storage_account_sku` : 스토리지 계정 생성
- `az storage account generate-sas --account-name 스토리지_계정_이름 -n 계정_이름 --permissions sas_permissions --expiry 만료_날짜 --services 스토리지_서비스 --resource-types 리소스_타입` : 특정 스토리지 계정에 대한 공유 액세스 서명을 생성
- `az storage account list -g azure_리소스_그룹` : 스토리지 계정 나열
- `az storage account delete -n 스토리지_계정_이름 -g azure_리소스_그룹` : 특정 저장소 계정 삭제

## Links
- More information: https://learn.microsoft.com/cli/azure/storage/account
