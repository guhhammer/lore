# pip wheel

패키지와 의존성의 wheel 아카이브 빌드.

## Usage
- `pip wheel 패키지` : 특정 패키지의 wheel 빌드
- `pip wheel -r 경로/대상/requirements.txt` : requirements 파일에 정의된 패키지들의 wheel 빌드
- `pip wheel 패키지 -w 경로/대상/디렉터리` : 특정 디렉터리에 wheel 빌드 결과 저장
- `pip wheel 패키지 --no-deps` : 의존성 없이 wheel 빌드
- `pip wheel 경로/대상/프로젝트` : 로컬 프로젝트로부터 wheel 빌드
- `pip wheel git+https://github.com/user/repo.git` : git 저장소로부터 wheel 빌드

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip_wheel/
