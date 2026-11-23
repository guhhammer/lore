# whatwaf

웹 애플리케이션 방화벽 및 보호 시스템 탐지 및 우회.

## Usage
- `whatwaf -u https://example.com --verbose` : 단일 uRL의 보호 시스템 탐지, 선택적으로 상세 출력 사용
- `whatwaf -t 숫자 -l 경로/대상/파일` : 파일에서 URL 목록을 병렬로 탐지 (한 줄에 하나의 URL)
- `whatwaf --proxy http://127.0.0.1:8080 --pl 경로/대상/파일 -u https://example.com` : 프록시를 통해 요청을 보내고 파일에서 사용자 정의 페이로드 목록 사용 (한 줄에 하나의 페이로드)
- `whatwaf --tor -p '페이로드1,페이로드2,...' -u https://example.com` : 토르를 통해 요청 전송 (토르가 설치되어야 함), 사용자 정의 p페이로드 사용 (쉼표로 구분)
- `whatwaf --ra --throttle 초 --timeout 초 -P --force-ssl -u http://example.com` : 랜덤 사용자 에이전트 사용, 대역폭 조절 및 타임아웃 설정, POST 요청 전송, HTTPS 연결 강제
- `whatwaf --wafs` : 탐지 가능한 모든 WAF 나열
- `whatwaf --tampers` : 사용 가능한 모든 변조 스크립트 나열

## Links
- More information: https://github.com/Ekultek/WhatWaf#basic-help-menu
