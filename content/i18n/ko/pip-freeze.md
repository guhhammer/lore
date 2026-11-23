# pip freeze

설치된 패키지를 요구 사항 형식으로 나열.

## Usage
- `pip freeze` : 설치된 패키지 나열
- `pip freeze >requirements.txt` : 설치된 패키지를 나열하고 `requirements.txt` 파일에 작성
- `pip freeze -l` : 가상 환경에서 설치된 패키지를 나열하고, 전역적으로 설치된 패키지를 제외
- `pip freeze --user` : 사용자 사이트에 설치된 패키지 나열
- `pip freeze --all` : `pip`, `distribute`, `setuptools`, `wheel`을 포함한 모든 패키지 나열 (기본적으로 생략됨)

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_freeze/
