# macchanger

네트워크 인터페이스 MAC 주소를 조작하는 명령줄 유틸리티.

## Usage
- `macchanger -s 인터페이스` : 인터페이스의 현재 및 영구 MAC 주소 보기
- `macchanger -r 인터페이스` : 인터페이스를 임의의 MAC으로 설정
- `macchanger -r -b 인터페이스` : 인터페이스를 임의의 MAC 주소로 설정하고, burned-in-address로 가장
- `macchanger -m XX:XX:XX:XX:XX:XX 인터페이스` : 인터페이스를 특정 MAC 주소로 설정
- `macchanger -l` : 알려진 모든 공급업체의 식별자(MAC 주소의 처음 세 바이트) 출력
- `macchanger -p 인터페이스` : 인터페이스를 영구 하드웨어 MAC 주소로 재설정

## Links
- More information: https://manned.org/macchanger
