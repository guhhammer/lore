# gtrash

다양한 기능을 제공하는 휴지통 관리 도구, `rm`과 `trash-cli`의 대안.

## Usage
- `gtrash put 경로/대상/파일_또는_디렉터리1 경로/대상/파일_또는_디렉터리2 ...` : 파일 또는 디렉터리를 휴지통으로 이동
- `gtrash find 정규_표현식` : 원래 경로가 `정규 표현식`과 일치하는 휴지통 파일 목록 표시
- `gtrash r` : 대화형 TUI를 열어 휴지통의 파일을 선택하고 복원
- `gtrash find 정규_표현식 --rm` : 원래 경로가 `정규 표현식`과 일치하는 휴지통 파일을 영구 삭제
- `gtrash prune --day 일` : 지정한 일수보다 오래된 휴지통 파일을 영구 삭제
- `gtrash summary` : 각 휴지통의 위치, 항목 수 및 전체 크기 표시

## Links
- More information: https://github.com/umlx5h/gtrash#usage
