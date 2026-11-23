# pass

비밀번호나 기타 민감한 데이터를 저장하고 읽기. 모든 데이터는 GPG로 암호화되며, Git 저장소로 관리됩니다.

## Usage
- `pass init gpg_id_1 gpg_id_2 ...` : 하나 이상의 GPG ID를 사용하여 저장소 초기화 (또는 재암호화)
- `pass insert -m 경로/대상/데이터` : 새 비밀번호 및 추가 정보 저장 (새 줄에서 `<Ctrl d>`를 눌러 완료)
- `pass edit 경로/대상/데이터` : 항목 편집
- `pass -c 경로/대상/데이터` : 비밀번호(데이터 파일의 첫 번째 줄)를 클립보드에 복사
- `pass` : 전체 저장소 트리 나열
- `pass generate -c 경로/대상/데이터 숫자` : 지정된 길이의 새로운 무작위 비밀번호 생성 및 클립보드에 복사
- `pass git init` : 새로운 Git 저장소 초기화 (pass에 의해 이루어진 모든 변경 사항은 자동으로 커밋됨)
- `pass git 명령` : 비밀번호 저장소를 대신하여 Git 명령 실행

## Links
- More information: https://git.zx2c4.com/password-store/about/
