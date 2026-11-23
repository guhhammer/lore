# pass otp

일회용 비밀번호(OTP) 토큰 관리를 위한 pass 확장 기능.

## Usage
- `pass otp insert 경로/대상/pass` : otpauth URI 토큰을 입력 받고 새로운 pass 파일 생성
- `pass otp append 경로/대상/pass` : otpauth URI 토큰을 입력 받고 기존 pass 파일에 추가
- `pass otp 경로/대상/pass` : pass 파일의 OTP 토큰을 사용하여 2FA 코드 출력
- `pass otp -c 경로/대상/pass` : pass 파일의 OTP 토큰을 사용하여 2FA 코드를 복사하고 출력하지 않음
- `pass otp uri -q 경로/대상/pass` : pass 파일에 저장된 OTP 토큰을 사용하여 QR 코드 표시
- `pass otp append -s -i 발행자_이름 -a 계정_이름 경로/대상/pass` : 발행자 및 계정을 지정하여 OTP 비밀 값을 입력 받고 기존 pass 파일에 추가 (적어도 하나는 지정해야 함)

## Links
- More information: https://manned.org/pass-otp
