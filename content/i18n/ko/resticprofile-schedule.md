# resticprofile schedule

백업 작업을 예약 및 백그라운드 실행.

## Usage
- `resticprofile schedule` : 기본 프로필의 백업 스케줄 설정
- `resticprofile --name "그룹_이름" schedule` : 지정한 프로필의 백업 스케줄 설정 (기본 프로필 이름은 "default")
- `resticprofile schedule --all` : 모든 프로필의 백업 스케줄 설정
- `resticprofile schedule --no-start` : 스케줄 작업을 설치한 후 바로 시작하지 않음
- `resticprofile status -n 프로필_이름` : 지정한 프로필의 예약된 작업 상태 표시
- `resticprofile run-schedule "backup@프로필_이름"` : 예약된 작업을 수동으로 실행 (시스템 스케줄러에서 사용)
