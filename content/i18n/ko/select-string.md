# select-string

PowerShell에서 문자열과 파일에서 텍스트를 찾습니다. 참고: 이 명령어는 PowerShell을 통해서만 사용할 수 있습니다. `Select-String`을 UNIX의 `grep`이나 Windows의 `findstr.exe`와 유사하게 사용할 수 있습니다.

## Usage
- `Select-String -Path "경로\대상\파일" -Pattern '검색_패턴'` : 파일 내에서 패턴 검색
- `Select-String -SimpleMatch "정확한_문자열" 경로\대상\파일` : 정확한 문자열 검색 (`regex` 비활성화)
- `Select-String -Path "*.ext" -Pattern '검색_패턴'` : 현재 디렉토리의 모든 `.ext` 파일에서 패턴 검색
- `Select-String --Context 2,3 "검색_패턴" 경로\대상\파일` : 패턴과 일치하는 줄 앞뒤의 지정된 줄 수 캡처
- `Get-Content 경로\대상\파일 | Select-String --NotMatch "검색_패턴"` : `stdin`에서 패턴과 일치하지 않는 줄 검색
