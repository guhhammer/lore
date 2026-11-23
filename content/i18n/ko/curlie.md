# curlie

`curl`의 프론트엔드로, `httpie`의 사용 편의성을 추가합니다.

## Usage
- `curlie httpbin.org/get` : GET 요청 전송
- `curlie post httpbin.org/post name=john age:=25` : POST 요청 전송
- `curlie get httpbin.org/get first_param==5 second_param==true` : 쿼리 매개변수를 포함한 GET 요청 전송 (예: `first_param=5&second_param=true`)
- `curlie get httpbin.org/get header-name:header-value` : 사용자 정의 헤더와 함께 GET 요청 전송

## Links
- More information: https://github.com/rs/curlie#usage
