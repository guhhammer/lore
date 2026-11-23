# wmpconfig

Windows Media Player (legacy)의 네트워크 공유를 관리. 참고: 이 명령은 관리자 권한이 필요함.

## Usage
- `wmpconfig HMEOn|HMEOff` : 네트워크 공유 기능 활성화 또는 비활성화
- `wmpconfig DisableHMEDevice mac_주소` : 지정한 MAC 주소의 네트워크 장치가 이 컴퓨터와 공유하지 못하도록 설정
- `wmpconfig RemoveHMEDevice mac_주소` : 지정한 네트워크 장치를 이 컴퓨터와의 공유 대상에서 제거
- `wmpconfig RestoreHMEDevice mac_주소` : 지정한 네트워크 장치와의 공유 복원
- `wmpconfig SetDVDParentalLevel 3` : DVD 콘텐츠 재생을 위한 자녀 보호 수준 설정
