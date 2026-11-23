# print.zsh

Z Shell (`zsh`) 내장 명령. `echo`와 유사하게 인수를 출력.

## Usage
- `print "Hello" "World"` : 입력 내용 출력
- `print -l "Line1" "Line 2" "Line3"` : 각 인수를 줄바꿈하여 출력ㄴ
- `print -n "Hello"; print "World"` : 마지막에 줄바꿈 없이 출력
- `print -e "Line 1\nLine2"` : 백슬래스 이스케이프 문자 사용
- `print -f "%s is %d years old.\n" "Alice" 30` : `printf` 형식으로 인수 출력 (쉘 간 호환성을 위해선, `printf` 명령 사용을 권장)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html
