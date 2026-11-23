# hashid

데이터 및 비밀번호 해시를 식별하는 Python3 프로그램.

## Usage
- `hashid` : `stdin`에서 해시를 식별 (입력, 복사 및 붙여넣기 또는 해시를 프로그램에 파이프 사용)
- `hashid 해시1 해시2 ...` : 하나 이상의 해시를 식별
- `hashid 경로/대상/해시.txt` : 파일의 해시를 식별 (한 줄에 하나의 해시)
- `hashid -e 해시` : 가능한 모든 해시 유형 표시 (salt된 해시를 포함)
- `hashid -m -j 해시` : `hashcat`의 모드 번호와 `john`의 해시 유형 형식 문자열을 표시
- `hashid -o 경로/대상/출력파일.txt 해시` : `stdout`으로 출력하는 대신 파일에 출력을 저장

## Links
- More information: https://github.com/psypanda/hashID#usage
