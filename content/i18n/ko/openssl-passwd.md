# openssl passwd

비밀번호 해시 생성.

## Usage
- `openssl passwd -5` : SHA256 알고리즘으로 비밀번호 해시 생성 (Linux 전용)
- `openssl passwd -apr1` : APR1 알고리즘으로 비밀번호 해시 생성
- `openssl passwd -apr1 -salt salt_문자열` : APR1 알고리즘에 salt 지정해 비밀번호 해시 생성
- `openssl passwd -apr1 -table` : APR1 알고리즘으로 비밀번호 해시를 생성하고 비밀번호와 해시를 함께 표시
- `echo -n "비밀번호" | openssl passwd -apr1 -stdin` : `stdin`에서 비밀번호를 읽어 해시를 생성

## Links
- More information: https://docs.openssl.org/master/man1/openssl-passwd/
