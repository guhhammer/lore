# get-dedupproperties

데이터 중복 제거 정보 가져오기. 참고: 이 명령은 PowerShell을 통해서만 사용할 수 있습니다.

## Usage
- `Get-DedupProperties -DriveLetter 'C'` : 드라이브의 데이터 중복 제거 정보 가져오기
- `Get-DedupProperties -FileSystemLabel 'Label'` : 드라이브 레이블을 사용하여 데이터 중복 제거 정보 가져오기
- `Get-DedupProperties -InputObject $(Get-Volume -DriveLetter 'E')` : 입력 객체를 사용하여 드라이브의 데이터 중복 제거 정보 가져오기

## Links
- More information: https://learn.microsoft.com/powershell/module/storage/get-dedupproperties
