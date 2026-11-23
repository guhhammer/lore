# uv tool

Python 패키지가 제공하는 명령을 설치하고 실행.

## Usage
- `uv tool run 명령` : 패키지를 설치하지 않고 명령 실행
- `uv tool install 패키지` : python 패키지를 시스템 전역에 설치
- `uv tool install "git+저장소_URL@v1.2.3"` : git 저장소의 특정 태그에서 설치
- `uv tool upgrade 패키지` : 설치된 Python 패키지 업그레이드
- `uv tool uninstall 패키지` : python 패키지 제거
- `uv tool list` : 시스템 전역에 설치된 Python 패키지 나열

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-tool
