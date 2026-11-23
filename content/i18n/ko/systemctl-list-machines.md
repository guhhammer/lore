# systemctl list-machines

호스트와 현재 실행 중인 모든 로컬 가상 머신 또는 컨테이너를 상태와 함께 나열.

## Usage
- `systemctl list-machines` : 모든 머신 (호스트 및 실행 중인 컨테이너 및 VM) 출력
- `systemctl list-machines 머신` : 특정 머신 출력
- `systemctl list-machines 머신_1 머신_2 ...` : 여러 머신 출력
- `systemctl list-machines 패턴` : 와일드카드 패턴(즉, `shell-globbing`)으로 머신 필터링
