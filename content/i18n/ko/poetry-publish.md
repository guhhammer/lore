# poetry publish

패키지를 원격 저장소에 배포.

## Usage
- `poetry publish` : 현재 패키지를 PyPI에 배포
- `poetry publish --build` : 패키지를 빌드한 후 배포
- `poetry publish -r 저장소_이름` : 지정한 저장소에 패키지 배포
- `poetry publish -u 사용자명 -p 비밀번호` : 지정한 인증 정보를 사용해 패키지 배포
- `poetry publish --dry-run` : 실제 배포 없이 수행될 작업 dry run
- `poetry publish --skip-existing` : 저장소에 이미 존재하는 파일은 건너뛰고 배포

## Links
- More information: https://python-poetry.org/docs/cli/#publish
