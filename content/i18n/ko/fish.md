# fish

사용자 친화적으로 설계된 명령줄 해석기인 Friendly Interactive SHell입니다.

## Usage
- `fish` : 대화형 쉘 세션을 시작
- `fish -N` : 시작 구성을 로드하지 않고 대화형 쉘 세션을 시작
- `fish -c "echo 'fish is executed'"` : 특정 명령을 실행
- `fish 경로/대상/스크립트.fish` : 특정 스크립트를 실행
- `fish -n 경로/대상/스크립트.fish` : 구문 오류가 있는지 특정 스크립트를 확인
- `echo "echo 'fish is executed'" | fish` : `stdin`에서 특정 명령을 실행
- `fish -P` : 쉘이 이전 기록에 액세스하거나 새 기록을 저장하지 않는 비공개 모드에서 대화형 쉘 세션을 시작
- `set -U -x 변수_이름 변수_값` : 쉘을 다시 시작해도 지속되는 환경 변수를 정의하고 내보냄 (기본 제공)

## Links
- More information: https://fishshell.com/docs/current/cmds/fish.html
