# piper

빠르고 로컬에서 동작하는 신경망 기반의 텍스트 음성 변환 시스템.

## Usage
- `echo 말할 내용 | piper -m 경로/대상/모델.onnx -f 출력파일.wav` : 텍스트 음성 변환 m모델을 사용하여 WAV f파일 출력(모델 경로에 대한 config 파일이 있을 경우)
- `echo '말할 내용' | piper -m 경로/대상/모델.onnx -c 경로/대상/모델.onnx.json -f 출력파일.wav` : m모델과 JSON c설정 파일을 지정하여 WAV f파일 출력
- `echo 'Warum?' | piper -m de_DE-thorsten_emotional-medium.onnx --speaker 1 -f 화남.wav` : 여러 명의 화자가 있는 음성에서 특정 화자를 ID 번호로 선택
- `echo 'Hello world' | piper -m en_GB-northern_english_male-medium.onnx --output-raw -f - | mpv -` : mpv 미디어 플레이어로 출력을 스트리밍
- `echo '두 배 속도로 말합니다. 드라마틱하게!' | piper -m foo.onnx --length_scale 0.5 --sentence_silence 2 -f 드라마.wav` : 두 배 빠르게 말하고 문장 사이에 큰 간격을 두기

## Links
- More information: https://rhasspy.github.io/piper-samples
