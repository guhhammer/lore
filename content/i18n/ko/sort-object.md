# sort-object

속성 값에 따라 개체를 정렬합니다. 참고: 이 명령은 PowerShell을 통해서만 사용할 수 있습니다.

## Usage
- `Get-ChildItem | Sort-Object` : 이름으로 현재 디렉터리 정렬
- `Get-ChildItem | Sort-Object -Descending` : 내림차순으로 현재 디렉터리 정렬
- `"a", "b", "a" | Sort-Object -Unique` : 중복 항목 제거
- `Get-ChildItem | Sort-Object -Property Length` : 파일 길이로 현재 디렉터리 정렬
- `Get-Process | Sort-Object -Property WS` : 메모리 사용량이 가장 많은 프로세스를 메모리 사용량 기준으로 정렬
