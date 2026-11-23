# git changelog

저장소 커밋 및 태그에서 변경 로그 보고서를 생성. `git-extras`의 일부.

## Usage
- `git changelog` : 최신 Git 태그 이후의 커밋 메시지로 기존 파일을 업데이트하거나 새 `History.md` 파일을 생성
- `git changelog -l` : 현재 버전의 커밋 나열
- `git changelog -l -s 2.1.0` : `2.1.0` 태그부터 현재까지의 커밋 범위 나열
- `git changelog -s 0.5.0 -f 1.0.0` : `0.5.0` 태그와 `1.0.0` 태그 사이의 커밋 범위를 보기 좋게 나열
- `git changelog --start-commit 0b97430 -f 1.0.0` : 커밋 `0b97430`과 태그 `1.0.0` 사이의 커밋 범위를 보기 좋게 나열
- `git changelog CHANGELOG.md` : 출력 파일로 `CHANGELOG.md` 지정
- `git changelog -p` : 현재 변경 로그 파일의 내용을 완전히 교체

## Links
- More information: https://manned.org/git-changelog
