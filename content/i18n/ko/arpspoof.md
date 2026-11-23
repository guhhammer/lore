# arpspoof

패킷을 가로채기 위해 ARP 응답을 위조합니다.

## Usage
- `sudo arpspoof -i wlan0 호스트_ip` : 호스트의 패킷을 가로채기 위해 i인터페이스의 모든 호스트를 중독
- `sudo arpspoof -i wlan0 -t 대상_ip 호스트_ip` : 호스트의 패킷을 가로채기 위해 i인터페이스의 t대상을 중독
- `sudo arpspoof -i wlan0 -r -t 대상_ip 호스트_ip` : 호스트의 패킷을 가로채기 위해 i인터페이스의 t대상과 호스트를 모두 중독

## Links
- More information: https://manned.org/arpspoof
