# az sshkey

가상 머신으로 SSH 공개 키를 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az sshkey create --name 이름 -g 리소스_그룹` : 새로운 SSH 키를 생성
- `az sshkey create --name 이름 -g 리소스_그룹 --public-key "@path/to/key.pub"` : 기존 SSH 키 업로드
- `az sshkey list` : 모든 SSH 공개 키를 나열
- `az sshkey show --name 이름 -g 리소스_그룹` : SSH 공개 키를 대한 정보 표시

## Links
- More information: https://learn.microsoft.com/cli/azure/sshkey
