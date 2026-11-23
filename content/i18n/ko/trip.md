# trip

네트워크 진단 도구. `traceroute`와 `ping`의 기능 결합. 네트워크 문제 분석 시 도움.

## Usage
- `sudo trip example.com` : 기본 설정으로 네트워크 경로 추적
- `trip example.com --unprivileged` : 권한 상승 없이 네트워크 경로를 추적 (지원되는 플랫폼만 해당)
- `sudo trip example.com --ipv6` : `IPv6`만 사용하여 네트워크 경로 추적
- `sudo trip example.com --protocol udp` : `udp` 프로토콜을 사용하여 네트워크 경로 추적
- `sudo trip example.com --protocol tcp --target-port 443` : `tcp` 경로 추적에 사용자 지정 대상 포트 `443` 사용
- `sudo trip example.com --protocol udp --source-port 5000` : `udp` 경로 추적에 사용자 지정 소스 포트 `5000` 사용

## Links
- More information: https://trippy.rs/reference/cli/
