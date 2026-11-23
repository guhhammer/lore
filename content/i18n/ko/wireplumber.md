# wireplumber

PipeWire를 위한 모듈식 세션/정책 관리자 및 PipeWire의 API를 감싸는 GObject 기반 고수준 라이브러리.

## Usage
- `systemctl enable wireplumber --user --now` : 사용자 세션과 함께 WirePlumber를 즉시 시작 (systemd 시스템의 경우)
- `wireplumber` : `pipewire`가 시작된 후 WirePlumber 실행 (non-systemd 시스템의 경우)
- `wireplumber -c 경로/대상/파일` : 다른 컨텍스트 구성 파일 지정
- `wireplumber -h` : 도움말 표시
- `wireplumber -v` : 버전 표시

## Links
- More information: https://pipewire.pages.freedesktop.org/wireplumber/
