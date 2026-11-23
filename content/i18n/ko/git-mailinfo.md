# git mailinfo

이메일 메시지에서 패치 및 작성자 정보를 추출.

## Usage
- `git mailinfo message|patch` : 이메일 메시지에서 패치 및 작성자 데이터 추출
- `git mailinfo -k message|patch` : 추출하지만 앞뒤 공백 제거
- `git mailinfo --scissors message|patch` : 본문에서 가위선 (예: "-->* --") 이전의 모든 내용을 제거하고 메시지 또는 패치 추출

## Links
- More information: https://git-scm.com/docs/git-mailinfo
