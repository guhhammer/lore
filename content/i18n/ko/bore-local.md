# bore local

Bore를 사용해 로컬 프록시를 원격 서버에 연결.

## Usage
- `bore local -t 원격_서버_주소 로컬_포트` : 로컬 포트를 원격 Bore 서버에 노출
- `bore local -l host -t 원격_서버_주소 로컬_포트` : `localhost` 대신 특정 로컬 호스트 노출
- `bore local -t 원격_서버_주소 -p 원격_포트 로컬_포트` : 원격 서버 포트를 명시적으로 지정
- `bore local -t 원격_서버_주소 -s 당신의_시크릿 로컬_포트` : 인증용 시크릿 사용
- `bore local -h` : 도움말 표시

## Links
- More information: https://github.com/ekzhang/bore#detailed-usage
