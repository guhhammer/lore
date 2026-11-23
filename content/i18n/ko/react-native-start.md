# react-native start

React Native 서버를 시작하는 명령줄 도구.

## Usage
- `react-native start` : 연결된 기기와 통신하는 서버 시작
- `react-native start --reset-cache` : 캐시를 초기화하여 메트로 번들러 시작
- `react-native start --port 3000` : 사용자 지정 포트(기본값은 8081)로 서버 시작
- `react-native start --verbose` : 자세한 모드로 서버 시작
- `react-native start --max-workers 개수` : 파일 변환 작업을 위한 최대 작업자 수 지정(기본값은 CPU 코어 수)
- `react-native start --no-interactive` : 대화형 모드 비활성화
