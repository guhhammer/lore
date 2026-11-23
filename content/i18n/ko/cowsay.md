# cowsay

ASCII 아트 (기본값은 소)를 사용해 말하거나 생각하는 메시지를 출력.

## Usage
- `cowsay "hello, world"` : ASCII 소가 "hello, world"라고 말하도록 출력
- `echo "hello, world" | cowsay` : `stdin`으로부터 입력받은 텍스트를 ASCII 소가 말하도록 출력
- `cowsay -l` : 사용 가능한 모든 아트 유형을 표시
- `cowsay -f 아트 "hello, world"` : 지정한 ASCII 아트가 "hello, world"라고 말하도록 출력
- `cowthink -d "I'm just a cow, not a great thinker..."` : 생각하는(dead) 상태의 ASCII 소를 출력
- `cowsay -e 문자열 "hello, world"` : 사용자 정의 눈 모양을 가진 ASCII 소가 "hello, world"라고 말하도록 출력

## Links
- More information: https://manned.org/cowsay
