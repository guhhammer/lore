# tailscale

개인 WireGuard 네트워크 서비스. `up`와 같은 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `sudo tailscale up` : tailscale에 연결
- `sudo tailscale down` : tailscale에서 연결 해제
- `tailscale ip` : 현재 Tailscale IP 주소 표시
- `tailscale ping ip|호스트명` : tailscale 계층에서 피어 노드를 핑하고 각 응답에 대한 경로 표시
- `tailscale netcheck` : 로컬 네트워크 상태를 분석하고 결과 표시
- `tailscale web` : tailscale을 제어하기 위한 웹 서버 시작
- `tailscale bugreport` : 문제 진단을 돕기 위한 공유 가능한 식별자 표시
- `tailscale 하위_명령 --help` : 하위 명령에 대한 도움말 표시

## Links
- More information: https://tailscale.com/kb/1080/cli
