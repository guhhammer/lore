# virtualenvwrapper

Python의 `virtualenv` 도구를 위한 간단한 래퍼 명령 그룹.

## Usage
- `mkvirtualenv 가상환경_이름` : 새로운 Python `virtualenv`를 `$WORKON_HOME`에 생성
- `mkvirtualenv --python /usr/local/bin/python3.8 가상환경_이름` : 특정 Python 버전에 대한 `virtualenv` 생성
- `workon 가상환경_이름` : 다른 `virtualenv` 활성화 또는 사용
- `deactivate` : `virtualenv` 중지
- `lsvirtualenv` : 모든 가상 환경 나열
- `rmvirtualenv 가상환경_이름` : `virtualenv` 제거
- `virtualenvwrapper` : 모든 virtualenvwrapper 명령 요약 보기

## Links
- More information: https://virtualenvwrapper.readthedocs.io/en/latest/command_ref.html
