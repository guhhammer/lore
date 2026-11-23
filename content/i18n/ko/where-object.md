# where-object

속성 값에 따라 컬렉션에서 개체를 선택합니다. 참고: 이 명령은 PowerShell을 통해서만 사용할 수 있습니다.

## Usage
- `Get-Alias | Where-Object -속성 이름 -eq 이름` : 별칭을 이름으로 필터링
- `Get-Service | Where-Object {$_.Status -eq "Stopped"}` : 현재 중지된 모든 서비스 나열. `$_` 자동 변수는 `Where-Object` cmdlet에 전달되는 각 개체를 나타냄
- `Get-Module -ListAvailable | Where-Object { $_.Name -NotLike "Microsoft*" -And $_.Name -NotLike "PS*" }` : 여러 조건 사용
