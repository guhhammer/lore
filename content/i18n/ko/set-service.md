# set-service

서비스를 시작, 중지 및 일시 중단하고 속성을 변경합니다. 이 명령어는 PowerShell을 통해서만 사용할 수 있습니다.

## Usage
- `Set-Service -Name 호스트명 -DisplayName "이름"` : 표시 이름 변경
- `Set-Service -Name 서비스명 -StartupType Automatic` : 서비스 시작 유형 변경
- `Set-Service -Name 서비스명 -Description "설명"` : 서비스 설명 변경
