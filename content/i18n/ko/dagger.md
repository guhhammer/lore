# dagger

로컬, CI 또는 클라우드에서 컨테이너 기반 파이프라인을 시랳ㅇ하는 프로그래밍 가능한 CI/CD 엔진.

## Usage
- `dagger init --sdk go|python|typescript 경로/대상/모듈` : 지정한 SDK로 새로운 모듈 초기화
- `dagger develop` : 로컬 모듈을 개발 가능 상태로 준비
- `dagger functions` : 현재 모듈에서 사용 가능한 함수 목록 표시
- `dagger call 함수_이름` : 현재 모듈의 함수 호출
- `dagger install github.com/user/repo` : git 저장소로부터 모듈 의존성 설치
- `dagger update` : 모듈 의존성 업데이트
- `dagger -h` : 도움말 표시
- `dagger version` : 버전 표시

## Links
- More information: https://docs.dagger.io/reference/cli
