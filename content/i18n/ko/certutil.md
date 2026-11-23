# certutil

NSS 데이터베이스와 기타 NSS 토큰 모두에서 키와 인증서를 관리.

## Usage
- `certutil -N -d .` : 현재 디렉터리(directory)에 새로운(New) 인증서 데이터베이스를 만듬
- `certutil -L -d .` : 데이터베이스의 모든 인증서를 나열
- `certutil -K -d . -f 경로/대상/패스워드_파일.txt` : 비밀번호 파일(file)을 지정하는 데이터베이스의 모든 개인 키(Keys)를 나열
- `certutil -A -n "서버_인증서" -t ",," -i 경로/대상/파일.crt -d .` : 닉네임(nickname), 신뢰 속성(trust attributes) 및 입력(input) CRT 파일을 지정하여 요청자 데이터베이스에 서명된 인증서를 추가(Add)
- `certutil -S -f 경로/대상/패스워드_파일.txt -d . -t ",," -c "서버_인증서" -n "서버_이름" -g 2048 -s "CN=공통_이름,O=조직"` : 지정한 키 크기(g)를 사용하여 인증서(certificate)에 subject alternative name 추가

## Links
- More information: https://manned.org/certutil
