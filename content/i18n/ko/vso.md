# vso

Vanilla OS를 위한 패키지 관리자, 시스템 업데이트 및 작업 자동화 도구.

## Usage
- `vso sys-upgrade check` : 호스트 시스템의 시스템 업데이트 확인
- `vso sys-upgrade upgrade --now` : 호스트 시스템을 지금 업그레이드
- `vso pico-init` : pico 하위 시스템 초기화 (패키지 관리에 사용됨)
- `vso install 패키지1 패키지2 ...` : 하위 시스템 내 애플리케이션 설치
- `vso remove 패키지1 패키지2 ...` : 하위 시스템에서 애플리케이션 제거
- `vso shell` : 하위 시스템의 셸에 진입
- `vso run 패키지` : 하위 시스템에서 애플리케이션 실행
- `vso config show` : VSO 구성 표시

## Links
- More information: https://github.com/Vanilla-OS/vanilla-system-operator
