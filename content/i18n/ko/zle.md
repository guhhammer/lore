# zle

Zsh Line Editor(ZLE) 위젯을 관리하는 도구. 참고: 일부 작업은 ZLE가 활성화된 상태(보통 사용자 정의 위젯 내부)에서만 사용할 수 있음.

## Usage
- `zle -l` : 사용자 정의 위젯 목록 출력(list)
- `zle -la` : 내장 위젯을 포함한 모든 위젯 목록 출력(list all)
- `zle -N 위젯_이름 optional_shell_function_name` : 새로운(New) 사용자 정의 위젯 생성 (기본 함수 이름은 위젯 이름과 동일)
- `zle -D 위젯_이름` : 위젯 삭제(Delete)
- `zle -A original_widget 별칭_이름` : 위젯 별칭(Alias) 생성
- `zle 위젯_이름` : 다른 위젯 함수 내부에서 위젯 호출 (ZLE 활성 상태 필요)
- `zle -U "텍스트"` : 입력된 것처럼 텍스트를 ZLE 입력 큐에 추가 (ZLE 활성 상태 필요)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Zsh-Line-Editor.html#Zle-Builtins
