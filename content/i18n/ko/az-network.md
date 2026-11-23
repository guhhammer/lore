# az network

Azure 네트워크 리소스를 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az network list-usages` : 구독 할당량에 대해 사용되는 지역의 네트워크 리소스 목록을 나열
- `az network vnet list` : 구독의 모든 가상 네트워크를 나열
- `az network vnet create --address-prefixes 10.0.0.0/16 -n 가상네트워크 -g 그룹_이름 --subnet-name 서브넷 --subnet-prefixes 10.0.0.0/24` : 가상 네트워크 만들기
- `az network nic update --accelerated-networking true -n 네트워크_인터페이스_카드 -g 리소스_그룹` : 네트워크 인터페이스 카드에 대한 가속화된 네트워킹 활성화

## Links
- More information: https://learn.microsoft.com/cli/azure/network
