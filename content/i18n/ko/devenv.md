# devenv

Nix를 사용하여 빠르고, 선언적이며, 재현 가능하고 구성 가능한 개발자 환경을 의미.

## Usage
- `devenv init` : 환경 초기화
- `devenv shell --impure` : 완화된 기밀성 (밀폐된 상태)로 개발 환경에 진입
- `devenv info --verbose` : 현재 환경에 대한 자세한 정보를 얻기
- `devenv up --config 경로/대상/파일` : `devenv`로 프로세스를 시작
- `devenv --clean --offline` : 환경 변수를 정리하고 오프라인 모드에서 쉘을 재입력
- `devenv gc` : 이전 쉘 세대를 삭제

## Links
- More information: https://devenv.sh/getting-started/#commands
