# uv pip

패키지 설치, 제거 및 관리를 위한 pip와 유사한 명령을 제공.

## Usage
- `uv pip install 패키지` : 패키지 설치
- `uv pip install -r requirements.txt` : requirements 파일에서 패키지 설치
- `uv pip install 패키지==1.2.3` : 지정한 버전의 패키지 설치
- `uv pip uninstall 패키지` : 패키지 제거
- `uv pip compile pyproject.toml -o requirements.txt` : `pyproject.toml`의 의존성을 잠금하여 `requirements.txt` 생성
- `uv pip list` : 설치된 패키지 목록 표시
- `uv pip show 패키지` : 설치된 패키지 정보 표시
- `uv pip sync requirements.txt` : requirements 파일과 정확히 일치하도록 환경 동기화 (필요한 패키지 설치/제거)

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-pip
