# bash

Bourne-Again SHell, an `sh`- 호환 명령 행 인터프리터.

## Usage
- `bash` : 대화형 쉘 시작하기
- `bash --norc` : 설정 파일 로딩 없이 대화형 쉘 시작하기
- `bash -c "echo 'bash가 실행되었습니다'"` : 특정 명령어(commands) 실행하기
- `bash 경로/대상/스크립트.sh` : 특정 스크립트 실행하기
- `bash -x 경로/대상/스크립트.sh` : 각 명령어 실행 전 명령어 인쇄하며 특정 스크립트 실행하기(Execute)
- `bash -e 경로/대상/스크립트.sh` : 첫 번째 에러(error)가 발생하면 중지되도록 하며 특정 스크립트 실행하기
- `echo "echo 'bash가 실행되었습니다'" | bash` : `stdin`에서 Bash 실행하기
- `bash -r` : 제한된(restricted) 쉘 세션을 시작

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#Invoking-Bash
