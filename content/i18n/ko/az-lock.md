# az lock

Azure 잠금 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az lock create -n 잠금_이름 -t ReadOnly` : 읽기 전용 구독 수준의 잠금을 생성
- `az lock create -n 잠금_이름 -g 그룹_이름 -t ReadOnly` : 읽기 전용 리소스 그룹 수준 잠금을 생성
- `az lock delete -n 잠금_이름` : 구독 수준 잠금을 해제
- `az lock delete -n 잠금_이름 -g 그룹_이름` : 리소스 그룹 수준의 잠금을 삭제
- `az lock list` : 구독 수준의 모든 잠금을 나열
- `az lock show -n 잠금_이름` : 특정 이름(n)으로 구독 수준 잠금 표시

## Links
- More information: https://learn.microsoft.com/cli/azure/lock
