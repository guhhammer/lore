# btrbk

btrfs 서브볼륨의 스냅샷 및 원격 백업 생성.

## Usage
- `sudo btrbk stats` : 구성된 서브볼륨 및 스냅샷에 대한 통계 출력
- `sudo btrbk list` : 구성된 서브볼륨 및 스냅샷 나열
- `sudo btrbk -v dryrun` : 표시된 변경 사항 없이 실행 시 어떤 일이 발생할지 출력
- `sudo btrbk --progress -v run` : 백업 루틴을 자세히 실행하고 진행 막대 표시
- `sudo btrbk snapshot` : 구성된 서브볼륨에 대해 스냅샷만 생성

## Links
- More information: https://digint.ch/btrbk/doc/readme.html
