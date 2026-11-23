# az aks

Azure Kubernetes Service (AKS) 클러스터 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az aks list -g 리소스_그룹` : AKS 클러스터 나열
- `az aks create -g 리소스_그룹 -n 이름 -c 개수 --node-vm-size 크기` : 새로운 AKS 클러스터 생성
- `az aks delete -g 리소스_그룹 -n 이름` : AKS 클러스터 삭제
- `az aks get-credentials -g 리소스_그룹 -n 이름` : AKS 클러스터에 대한 접근 자격 증명을 가져옴
- `az aks get-upgrades -g 리소스_그룹 -n 이름` : AKS 클러스터에 사용할 수 있는 업그레이드 버전 가져오기

## Links
- More information: https://learn.microsoft.com/cli/azure/aks
