# resticprofile

restic 백업을 위한 설정 프로필을 관리.

## Usage
- `resticprofile` : 저장된 모든 스냅샷 목록 표시
- `resticprofile backup` : 기본 프로필로 백업을 실행
- `resticprofile -n "프로필_이름" backup` : 지정한 프로필로 백업 실행 (기본 프로필 이름은 "default")
- `resticprofile --dry-run backup` : dry-run 모드로 실행하고 내부적으로 실행될 restic 명령 표시
- `resticprofile profiles` : 설정 파일에 정의된 모든 프로필 이름 표시
- `resticprofile generate --bash-completion|--zsh-completion` : 쉘 자동 완성 스크립트 생성
- `resticprofile show -n "프로필_이름"` : 지정한 프로필의 모든 세부 정보 표시
