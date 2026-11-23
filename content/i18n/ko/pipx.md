# pipx

Python 애플리케이션을 격리된 환경에서 설치하고 실행.

## Usage
- `pipx run pycowsay moo` : 임시 가상 환경에서 앱 실행
- `pipx install 패키지` : 가상 환경에 패키지 설치 및 진입점을 경로에 추가
- `pipx list` : 설치된 패키지 나열
- `pipx run --spec httpx-cli httpx http://www.github.com` : 실행 파일과 다른 패키지 이름으로 임시 가상 환경에서 앱 실행
- `pipx inject 패키지 의존성1 의존성2 ...` : 기존 가상 환경에 의존성 추가
- `pipx install --pip-args='pip-인자' 패키지` : pip 인자를 사용하여 가상 환경에 패키지 설치
- `pipx upgrade-all|uninstall-all|reinstall-all` : 모든 설치된 패키지 업그레이드/재설치/제거

## Links
- More information: https://manned.org/pipx
