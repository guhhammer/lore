# az storage

Azure Cloud Storage 리소스를 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az storage account create -g 그룹_이름 -n 계정_이름 -l 위치 --sku account_sku` : 위치(location)를 지정하는 스토리지 계정을 생성
- `az storage account list -g 그룹_이름` : 리소스 그룹의 모든 스토리지 게정을 나열
- `az storage account keys list -g 그룹_이름 -n 계정_이름` : 스토리지 계정에 대한 액세스 키를 나열
- `az storage account delete -g 그룹_이름 -n 계정_이름` : 스토리지 계정 삭제
- `az storage account update --min-tls-version TLS1_0|TLS1_1|TLS1_2 -g 그룹_이름 -n 계정_이름` : 스토리지 계정에 대한 최소 TLS 버전 설정을 업데이트

## Links
- More information: https://learn.microsoft.com/cli/azure/storage
