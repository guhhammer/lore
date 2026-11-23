# dumpcap

네트워크 트래픽 덤프 도구.

## Usage
- `dumpcap -D` : 사용 가능한 인터페이스 표시
- `dumpcap -i 1` : 특정 인터페이스에서 패킷을 캡처
- `dumpcap -i 1 -w 경로/대상/출력_파일.pcapng` : 특정 위치로 패킷을 캡처
- `dumpcap -i 1 -w 경로/대상/출력_파일.pcapng -b filesize:500000 -b files:10` : 특정 크기의 특정 최대 파일 제한을 사용해 링 버퍼에 쓰기

## Links
- More information: https://www.wireshark.org/docs/man-pages/dumpcap.html
