# poetry build

Poetry 패키지를 tarball과 wheel 형식으로 빌드.

## Usage
- `poetry build` : 프로젝트의 tarball과 wheel 패키지 빌드
- `poetry build -f wheel` : wheel 패키지 빌드
- `poetry build -f sdist` : 소스 배포판(sdist) 빌드
- `poetry build --clean` : 빌드 전에 출력 디렉터리 정리
- `poetry build -o 경로/대상/디렉터리` : 출력 디렉터리 지정

## Links
- More information: https://python-poetry.org/docs/cli/#build
