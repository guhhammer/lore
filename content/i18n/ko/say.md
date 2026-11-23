# say

텍스트를 음성으로 변환.

## Usage
- `say "나는 자전거 타는 것을 좋아해."` : 문구를 소리내어 말하기
- `say --input-file 파일명.txt` : f파일을 소리내어 읽기
- `say --voice 음성 --rate 분당_단어_수 "미안해 데이브, 그렇게 할 수 없어."` : 사용자 지정 음성과 속도로 문구 말하기
- `say --voice "?"` : 사용 가능한 음성 목록 나열 (다양한 언어로 말하는 음성)
- `say --voice Zosia "Litwo, ojczyzno moja!"` : 폴란드어로 말하기
- `say --output-file 파일명.aiff "Here's to the Crazy Ones."` : 음성 텍스트의 오디오 파일 생성

## Links
- More information: https://keith.github.io/xcode-man-pages/say.1.html
