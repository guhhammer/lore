# git maintenance

Git 저장소 데이터를 최적화하기 위한 작업 실행.

## Usage
- `git maintenance register` : 현재 저장소를 사용자의 목록에 등록하여 매일 유지 관리 작업 실행
- `git maintenance start` : 현재 저장소에서 유지 관리 작업 시작
- `git maintenance stop` : 현재 저장소의 백그라운드 유지 관리 일정 중지
- `git maintenance unregister` : 현재 저장소를 사용자의 유지 관리 저장소 목록에서 제거
- `git maintenance run --task commit-graph|gc|incremental-repack|loose-objects|pack-refs|prefetch` : 현재 저장소에서 특정 유지 관리 작업 실행

## Links
- More information: https://git-scm.com/docs/git-maintenance
