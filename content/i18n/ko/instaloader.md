# instaloader

Instagram에서 사진, 비디오, 캡션 및 기타 메타데이터를 다운로드. 참고: 고품질 미디어 다운로드를 위해 Instagram 로그인 정보가 필요합니다.

## Usage
- `instaloader 프로필_이름` : 프로필 다운로드
- `instaloader --highlights 프로필_이름` : 하이라이트 다운로드
- `instaloader -q -G 프로필_이름` : 지오태그가 포함된 게시물 다운로드 (사용 가능한 경우) 및 사용자 상호작용 억제
- `instaloader --user-agent 사용자_에이전트 프로필_이름` : HTTP 요청에 사용할 사용자 에이전트 지정
- `instaloader -l 사용자명 -p 비밀번호 프로필_이름` : 로그인 정보 지정 및 게시물 다운로드 (비공개 프로필에 유용)
- `instaloader -F 프로필_이름` : 처음 다운로드된 파일이 발견되면 대상을 건너뜀 (Instagram 아카이브 업데이트에 유용)
- `instaloader -l 사용자명 -p 비밀번호 -s --igtv 프로필_이름` : 스토리 및 IGTV 비디오 다운로드 (로그인 필요)
- `instaloader -l 사용자명 -p 비밀번호 -s --igtv --highlights 프로필_이름` : 모든 유형의 게시물 다운로드 (로그인 필요)

## Links
- More information: https://instaloader.github.io/cli-options.html
