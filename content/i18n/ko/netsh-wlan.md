# netsh wlan

무선 네트워크를 관리.

## Usage
- `netsh wlan show networks` : 사용 가능한 모든 무선 네트워크 목록 표시
- `netsh wlan connect name=ssid` : 지정한 SSID의 무선 네트워크에 연결
- `netsh wlan disconnect` : 현재 연결된 무선 네트워크 연결 해제
- `netsh wlan show interfaces` : 현재 무선 네트워크 인터페이스와 상태 표시
- `netsh wlan export profile name=ssid folder=C:\경로\대상\폴더 key=clear` : 무선 네트워크 프로필을 XML 파일로 내보내기
- `netsh wlan delete profile name=ssid` : 저장된 무선 네트워크 프로필 삭제
- `netsh wlan set hostednetwork mode=allow ssid=ssid key=비밀번호` : 호스트 네트워크를 활성화 (PC를 Wi-Fi 핫스팟으로 설정)
- `netsh wlan start hostednetwork` : 호스트 네트워크 시작

## Links
- More information: https://www.serverwatch.com/guides/netsh-commands/
