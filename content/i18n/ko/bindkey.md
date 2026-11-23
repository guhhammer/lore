# bindkey

Z 셸에 단축키를 추가합니다.

## Usage
- `bindkey` : 등록된 단축키 목록을 표시
- `bindkey "^k" kill-line` : 특정 명령에 단축키를 바인딩
- `bindkey -s '^o' 'cd ..\n'` : 핫키를 특정 키 시퀀스(sequence)에 바인딩
- `bindkey -l` : 키맵의 목록(list)을 출력
- `bindkey -M main` : 키맵(keyMap)에서 단축키 보기
- `bindkey -v` : vi 모드를 활성화
- `bindkey -e` : emacs 모드를 활성화 (기본 모드)
- `bindkey -lL main | grep -Eo 'viins|emacs'` : 어떤 모드가 활성화되어 있는지 확인 (vi 또는 emacs)

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Zsh-Line-Editor.html#Zle-Builtins
