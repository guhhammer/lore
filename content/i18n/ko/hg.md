# hg

Mercurial - 분산 버전 관리 시스템. `commit`와 같은 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `hg init` : 빈 Mercurial 저장소 생성
- `hg clone https://example.com/repo` : 인터넷에서 원격 Mercurial 저장소 복제
- `hg status` : 로컬 저장소 상태 확인
- `hg add` : 새로운 파일을 모두 다음 커밋에 추가
- `hg commit -m 메시지_문자열` : 변경 사항을 버전 히스토리에 커밋
- `hg push` : 로컬 변경 사항을 원격 저장소로 푸시
- `hg pull` : 원격 저장소의 변경 사항 가져오기
- `hg update -C; hg purge` : 최신 커밋 상태로 완전히 되돌리기

## Links
- More information: https://www.mercurial-scm.org/help/commands
