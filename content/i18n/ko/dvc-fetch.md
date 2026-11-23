# dvc fetch

원격 저장소에서 DVC로 추적된 파일 및 디렉토리를 다운로드.

## Usage
- `dvc fetch` : 기본 원격 업스트림 저장소(설정된 경우)에서 최신 변경사항 가져오기
- `dvc fetch -r 원격_이름` : 특정 원격 업스트림 저장소에서 변경사항 가져오기
- `dvc fetch 대상/들` : 특정 대상의 최신 변경사항 가져오기
- `dvc fetch -a -T` : 모든 브랜치 및 태그의 변경사항 가져오기
- `dvc fetch -A` : 모든 커밋의 변경사항 가져오기

## Links
- More information: https://doc.dvc.org/command-reference/fetch
