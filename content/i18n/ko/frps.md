# frps

리버스 프록스 서비스를 빠르게 설정. `frp`의 부분.

## Usage
- `frps` : 기본 구성 파일(현재 디렉터리의 `frps.ini`로 가정)을 사용하여 서비스를 시작
- `frps -c 경로/대상/frps.toml` : 현재 디렉터리에서 최신 TOML 구성 파일 (`frps.ini` 대신 `frps.toml`)을 사용하여 서비스를 시작
- `frps -c 경로/대상/파일` : 특정 구성 파일을 사용하여, 서비스를 시작
- `frps verify -c 경로/대상/파일` : 구성 파일이 유효한지 확인
- `frps completion bash|fish|powershell|zsh` : bash, fish, PowerShell 또는 Zsh에 대한 자동 완성 설정 스크립트를 출력
- `frps -v` : 버전 표시

## Links
- More information: https://github.com/fatedier/frp
