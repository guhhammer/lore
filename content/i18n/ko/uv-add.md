# uv add

패키지 의존성을 `pyproject.toml` 파일에 추가.

## Usage
- `uv add 패키지` : 패키지의 최신 버전 추가
- `uv add 패키지1 패키지2 ...` : 여러 패키지 추가
- `uv add 패키지>=1.2.3` : 버전 요구 사항을 지정해 패키지 추가
- `uv add --optional optional 패키지1 패키지2 ...` : 배포 시 포함되는 선택적 의존성 그룹에 패키지 추가
- `uv add --group 그룹 패키지1 패키지2 ...` : 배포 시 포함되지 않는 로컬 의존성 그룹에 패키지 추가
- `uv add --dev 패키지1 패키지2 ...` : dev 그룹에 패키지 추가 (`--group dev`의 축약 표현)
- `uv add --editable 경로/대상/패키지/` : 패키지를 편집 가능 모드로 추가
- `uv add 패키지 --extra 추가_기능` : 패키지 설치 시 추가 기능 활성화 (여러 번 지정 가능)

## Links
- More information: https://peps.python.org/pep-0508/
