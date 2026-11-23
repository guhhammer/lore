# fossil commit

파일을 Fossil 저장소에 커밋.

## Usage
- `fossil ci` : 현재 체크아웃의 모든 변경 사항을 포함하는 새로운 버전을 만듬; 사용자에게 설명을 요청하는 메시지가 표시됨
- `fossil ci -m "코멘트"` : 지정된 설명을 사용하여, 현재 체크아웃의 모든 변경 사항을 포함하는 새 버전을 만듬
- `fossil ci -M 경로/대상/커밋_메시지` : 특정 파일에서 읽은 설명과 함께 현재 체크아웃의 모든 변경 사항을 포함하는 새 버전을 생성
- `fossil ci 경로/대상/파일1 경로/대상/파일2 ...` : 지정한 파일의 변경 사항을 포함하는 새로운 버전 생성 후, 사용자에게 주석 입력을 요청

## Links
- More information: https://fossil-scm.org/home/help/commit
