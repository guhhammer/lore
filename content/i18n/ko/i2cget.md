# i2cget

I2C 장치의 레지스터 값을 읽음. 참고: 모든 주소는 16진수로 지정해야 함.

## Usage
- `i2cget i2cbus 장치_주소 레지스터_주소` : I2C 장치의 레지스터 값 읽기
- `i2cget -y i2cbus 장치_주소 레지스터_주소` : 확인 메시지 없이 I2C 장치의 레지스터 값 읽기
- `i2cget i2cbus 장치_주소 레지스터_주소 b|w|c|s|i` : 지정한 모드로 I2C 장치의 레지스터 값 읽기

## Links
- More information: https://manned.org/i2cget
