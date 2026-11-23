# uv export

프로젝트의 잠금 파일을 다른 형식으로 내보내기.

## Usage
- `uv export --format requirements-txt -o requirements.txt` : 의존성을 `requirements.txt` 파일로 내보내기
- `uv export --format pylock.toml` : 의존성을 `pylock.toml` 형식으로 내보내기
- `uv export --no-dev` : 프로덕션 의존성만 내보내기 (개발 의존성 제외)
- `uv export --extra 그룹_이름` : 지정한 선택적 의존성 그룹을 포함하여 내보내기
- `uv export --all-extras` : 모든 선택적 의존성을 포함하여 내보내기
- `uv export --group 그룹_이름` : 지정한 의존성 그룹을 포함하여 내보내기
- `uv export --no-hashes` : 해시를 제외하고 내보내기
- `uv export --package 패키지_이름` : workspace의 지정한 패키지에 대한 의존성 내보내기

## Links
- More information: https://docs.astral.sh/uv/reference/cli/#uv-export
