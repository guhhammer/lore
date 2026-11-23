# bridge

네트워크 브릿지와 인터페이스를 조회 및 관리하는 도구.

## Usage
- `bridge l` : 모든 브릿지와 인터페이스 목록 출력
- `bridge v` : 포트 VLAN 정보 출력
- `sudo bridge v a dev lanX vid vlan_id pvid tagged|untagged` : VLAN 할당
- `sudo bridge v d dev lanX vid vlan_id` : 포트에서 VLAN 제거
- `bridge mo` : 브릿지 인터페이스 변경 사항 모니터링
- `bridge h` : 도움말 표시

## Links
- More information: https://manned.org/bridge
