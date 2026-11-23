# rdpsign

원격 데스크톱 프로토콜(RDP) 파일을 서명하는 도구입니다.

## Usage
- `rdpsign 경로\대상\파일.rdp` : RDP 파일 서명
- `rdpsign 경로\대상\파일.rdp /sha265 해시` : 특정 sha256 해시를 사용하여 RDP 파일 서명
- `rdpsign 경로\대상\파일.rdp /q` : 최소 출력 설정
- `rdpsign 경로\대상\파일.rdp /v` : 자세한 경고, 메시지 및 상태 출력
- `rdpsign 경로\대상\파일.rdp /l` : 파일을 업데이트하지 않고 출력 결과를 `stdout`에 표시하여 서명을 테스트
