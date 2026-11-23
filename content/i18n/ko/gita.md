# gita

여러 Git 저장소를 나란히 관리.

## Usage
- `gita ll` : 등록된 모든(a[ll]) 저장소 요약 정보 표시
- `gita st` : 등록된 모든 저장소 상태([st]atus) 표시
- `gita add 경로/대상/저장소1 경로/대상/저장소2 ...` : 하나 이상의 저장소를 gita에 등록
- `gita pull` : 등록된 모든 저장소 업데이트 Pull
- `gita super 저장소_이름 git_명령` : 특정 저장소에서 Git 명령 실행
- `gita group add -n 그룹_이름 저장소1 저장소2 ... && gita context 그룹_이름` : 저장소 그룹 생성 후 활성 컨텍스트로 설정
- `gita fetch 저장소1 저장소2 ...|그룹1 그룹2 ...` : 하나 이상의 저장소 또는 그룹 업데이트 fetch
- `gita freeze` : 모든 저장소 정보 표시

## Links
- More information: https://github.com/nosarthur/gita
