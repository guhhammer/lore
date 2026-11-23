# airodump-ng

패킷을 캡처하고, 무선 네트워크에 대한 정보를 표시. `aircrack-ng`의 일부.

## Usage
- `sudo airodump-ng 인터페이스` : 2.4GHz 대역의 무선 네트워크에 대한 패킷을 캡처하고 정보를 표시
- `sudo airodump-ng 인터페이스 --band a` : 5GHz 대역의 무선 네트워크에 대한 패킷을 캡처하고 정보를 표시
- `sudo airodump-ng 인터페이스 --band abg` : 2.4GHz 및 5GHz 대역 모두에서 무선 네트워크에 대한 패킷을 캡처하고 정보를 표시
- `sudo airodump-ng --channel channel --write 경로/대상/파일 --bssid mac interface` : MAC 주소와 채널을 통해 패킷을 캡처하고, 무선 네트워크에 대한 정보를 표시 및 출력 결과를 파일에 저장

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airodump-ng
