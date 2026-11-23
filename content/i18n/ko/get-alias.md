# get-alias

현재 PowerShell 세션에서 명령 별칭을 나열하고 가져옵니다. 이 명령은 PowerShell에서만 실행할 수 있습니다.

## Usage
- `Get-Alias` : 현재 세션의 모든 별칭 나열
- `Get-Alias 명령_별칭` : 별칭이 가리키는 명령 이름 가져오기
- `Get-Alias -Definition 명령` : 특정 명령에 할당된 모든 별칭 나열
- `Get-Alias abc* -Exclude *def` : `abc`로 시작하고 `def`로 끝나지 않는 별칭 나열
