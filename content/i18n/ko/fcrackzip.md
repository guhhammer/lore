# fcrackzip

암호화된 ZIP 압축 파일의 비밀번호를 빠르게 복구하는 도구.

## Usage
- `fcrackzip -D -p 경로/대상/단어목록.txt 경로/대상/파일.zip` : 단어 목록을 사용하여 사전 공격 수행
- `fcrackzip -v -u -D -p 경로/대상/단어목록.txt 경로/대상/파일.zip` : 자세한(Verbose) 출력을 표시하고 추측한 비밀번호를 검증하며 사전 공격 수행
- `fcrackzip -b -c a -l 최소-최대 경로/대상/파일.zip` : 지정한 길이 범위 내에서 소문자를 사용하여 비밀번호 무차별 대입 공격 수행
- `fcrackzip -b -c a1 경로/대상/파일.zip` : 소문자와 숫자를 사용하여 비밀번호 무차별 대입 공격 수행

## Links
- More information: https://github.com/hyc/fcrackzip
