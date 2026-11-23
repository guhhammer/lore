# npm hook

패키지를 위한 `npm` 레지스트리 훅을 관리하는 명령어. 참고: 이 명령어는 현재 deprecated 상태입니다.

## Usage
- `npm hook ls` : 활성화된 모든 훅 목록 출력
- `npm hook add 패키지_이름 이벤트 타겟_주소` : 특정 패키지에 새로운 훅 추가
- `npm hook rm 훅_아이디` : ID로 특정 훅 제거
- `npm hook update 훅_아이디 타겟_주소` : 새로운 정보로 훅 업데이트

## Links
- More information: https://docs.npmjs.com/cli/v10/hook/
