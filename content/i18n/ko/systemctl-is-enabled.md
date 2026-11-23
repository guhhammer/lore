# systemctl is-enabled

유닛 파일이 활성화되어 있는지 확인.

## Usage
- `systemctl is-enabled 유닛1 유닛2 ...` : 활성화 상태 출력
- `systemctl is-enabled 유닛 -q` : 출력 없이 종료 코드로만 확인
- `systemctl is-enabled 유닛 -l` : 설치된 타겟과 심볼릭 링크 경로까지 상세 출력
