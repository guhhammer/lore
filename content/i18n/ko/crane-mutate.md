# crane mutate

이미지 라벨과 주석을 수정. 컨테이너를 레지스트리에 푸시해야 하며, 매니페스트가 레지스트리에서 업데이트.

## Usage
- `crane mutate -a/-l annotation/label` : 새로운 주석을 설정 (기본값 [])
- `crane mutate --append/--cmd/--entrypoint/-e/--exposed-ports var1 var2 ...` : 이미지에 추가할 tarball/command/entrypoint/environment variable/exposed-ports의 경로를 지정
- `crane mutate -o 경로/대상/tarball` : 결과 이미지의 새로운 tarball 경로
- `crane mutate --set-platform 플랫폼_이름` : 변형된 이미지를 푸시하기 위한 `os/arch/variant:osversion,platform` 형식의 저장소
- `crane mutate -t 태그_이름` : 변형된 이미지에 적용할 새로운 태그를 참조
- `crane mutate -u 사용자명` : 새로운 사용자 설정
- `crane mutate -w 경로/대상/작업디렉토리` : 설정할 새로운 작업 디렉토리 설정
- `crane mutate -h` : 도움말 표시
