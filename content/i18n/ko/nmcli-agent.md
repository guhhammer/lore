# nmcli agent

`nmcli`를 NetworkManager 비밀 요원이나 polkit 요원으로 실행.

## Usage
- `nmcli a s` : `nmcli`를 비밀 요원으로 등록하고 비밀 요청 수신 대기
- `nmcli a p` : `nmcli`를 polkit 요원으로 등록하고 권한 요청 수신 대기
- `nmcli a a` : `nmcli`를 비밀 요원 및 polkit 요원으로 등록
