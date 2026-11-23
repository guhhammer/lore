# history

명령줄 히스토리를 관리.

## Usage
- `history` : 명령줄 히스토리를 줄 번호와 함께 출력
- `history 20` : 최근 20개의 명령어 출력 (Zsh에서는 20번째 이후의 모든 명령어 출력)
- `history -d|f|i|E` : 다양한 형식의 타임스탬프와 함께 히스토리 출력 (Zsh 전용)
- `history -c` : 명령어 히스토리 목록 삭제(clear)
- `history -w` : 현재 Bash 쉘의 히스토리로 히스토리 파일을 덮어쓰기(Overwrite) (보통 `history -c`와 함께 사용하여 히스토리를 완전히 삭제할 때 사용)
- `history -d 오프셋` : 지정한 위치(offset)의 히스토리 항목 삭제(delete)
- `history -s 명령어` : 명령어를 실행하지 않고 히스토리에 추가
- `<Space>명령어` : 명령어 앞에 공백을 추가하여 히스토리에 기록하지 않고 실행

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-history
