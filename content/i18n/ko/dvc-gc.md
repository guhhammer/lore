# dvc gc

캐시 또는 원격 저장소에서 사용되지 않는 파일 및 디렉토리를 제거.

## Usage
- `dvc gc -w` : 현재 작업 공간에서 참조하는 버전만 남기고 캐시에서 가비지 수집
- `dvc gc -a -T -a` : 브랜치, 태그, 커밋에서 참조하는 버전만 남기고 캐시에서 가비지 수집
- `dvc gc -a -c` : 기본 클라우드 원격 저장소를 포함하여 캐시에서 가비지 수집 (설정된 경우)
- `dvc gc -a -c -r 원격_이름` : 특정 클라우드 원격 저장소를 포함하여 캐시에서 가비지 수집

## Links
- More information: https://doc.dvc.org/command-reference/gc
