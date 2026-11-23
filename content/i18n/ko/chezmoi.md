# chezmoi

Go로 작성된, 다중 머신 도트파일 관리자.

## Usage
- `chezmoi init` : `chezmoi`를 설정하고, `~/.local/share/chezmoi`에 Git 저장소를 만듬
- `chezmoi init 레포지토리_주소` : git 저장소의 기존 도트 파일에서 `chezmoi`를 설정
- `chezmoi add 경로/대상/도트파일1 경로/대상/도트파일2 ...` : 하나 이상의 도트 파일 추적을 시작
- `chezmoi re-add 경로/대상/도트파일1 경로/대상/도트파일2 ...` : 로컬 변경 사항으로 저장소 업데이트
- `chezmoi edit 경로/대상/도트파일_또는_심볼릭링크` : 추적된 도트파일의 소스 상태를 편집
- `chezmoi diff` : 대기 중인 변경사항 보기
- `chezmoi apply` : 변경 사항을 적용
- `chezmoi update` : 원격 저장소에서 변경 사항을 가져와 적용

## Links
- More information: https://www.chezmoi.io/reference/
