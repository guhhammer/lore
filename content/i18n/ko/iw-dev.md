# iw dev

무선 장치를 표시하고 조작합니다.

## Usage
- `sudo iw dev wlanX set type monitor` : 장치를 모니터 모드로 설정(인터페이스는 먼저 종료되어야 합니다. `ip link`도 참조)
- `sudo iw dev wlanX set type managed` : 장치를 관리 모드로 설정(인터페이스는 먼저 종료되어야 합니다)
- `sudo iw dev wlanX set channel 채널_번호` : 장치의 WiFi 채널 설정(장치는 먼저 인터페이스가 활성화된 상태에서 모니터 모드여야 합니다)
- `sudo iw dev wlanX set freq 주파수_단위_메가헤르츠` : 장치의 WiFi 주파수 설정(Mhz 단위)(장치는 먼저 인터페이스가 활성화된 상태에서 모니터 모드여야 합니다)
- `iw dev wlanX station dump` : 모든 알려진 스테이션 정보 표시
- `sudo iw dev wlanX interface add "가상_인터페이스_이름" type monitor addr 12:34:56:aa:bb:cc` : 특정 MAC 주소로 모니터 모드의 가상 인터페이스 생성
- `sudo iw dev "가상_인터페이스_이름" del` : 가상 인터페이스 삭제
