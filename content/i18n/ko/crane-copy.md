# crane copy

다이제스트 값을 유지하면서 소스에서 대상으로 원격 이미지를 효율적으로 복사.

## Usage
- `crane cp 소스 대상` : 소스에서 대상으로 이미지를 복사
- `crane cp 소스 대상 -a` : 모든 태그 복사
- `crane cp 소스 대상 -j int` : 최대 동시 복사본 수를 설정, 기본값은 GOMAXPROCS
- `crane cp 소스 대상 -n` : 대상의 기존 태그를 덮어쓰지 않음
- `crane cp -h` : 도움말 표시
