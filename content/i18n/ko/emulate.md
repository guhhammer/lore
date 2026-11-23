# emulate

특정 쉘의 동작을 모방하도록 쉘 옵션 재설정. 쉘 함수가 기본 설정으로 실행되도록 보장하는 데에도 사용됨.

## Usage
- `emulate` : 현재 에뮬레이션 모드 출력
- `emulate bash` : 현재 쉘 세션 설정을 Bash처럼 동작하도록 조정
- `emulate ksh -c "echo hi"` : 임시 에뮬레이션 환경에서 명령(command) 실행
- `emulate zsh -R` : 기본 Zsh 환경을 에뮬레이션하고 가능하면 옵션 재설정(Resetting)
- `my_function() { emulate -L zsh|sh|ksh|csh; ls }` : 현재 함수 범위 내에서만 에뮬레이션 적용

## Links
- More information: https://github.com/antonio/zsh-config/blob/master/help/emulate
