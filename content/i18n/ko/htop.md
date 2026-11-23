# htop

실행 중인 프로세스에 대한 동적인 실시간 정보를 표시. `top`의 향상된 버전.

## Usage
- `htop` : `htop` 실행
- `htop -u 사용자명` : 특정 사용자가 소유한 프로세스만 표시하는 `htop`실행
- `htop -t` : 프로세스를 트리 구조로 표시해 부모-자식 관계를 확인
- `htop -s sort_item` : 지정한 기준(`sort_item`)으로 프로세스 정렬 (사용 가능한 옵션은 `htop --sort help` 참고)
- `htop -d 50` : 업데이트 간격을 지정하여 `htop` 실행 (0.1초 단위, 예: 50 = 5초)
- `htop --readonly` : 시스템 및 프로세스 변경 기능을 모두 비활성화
- `<F1>|<?>` : `htop` 실행 중 대화형 명령어 확인
- `<Tab>` : 다른 탭으로 전환

## Links
- More information: https://manned.org/htop
