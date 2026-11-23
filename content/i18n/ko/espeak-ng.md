# espeak-ng

다국어를 지원하는 소프트웨어 음성 합성기.

## Usage
- `espeak-ng "text"` : 텍스트를 음성으로 출력
- `echo "text" | espeak-ng` : `stdin`의 텍스트를 음성으로 출력
- `espeak-ng -f 경로/대상/파일` : 파일(file) 내용을 음성으로 출력
- `espeak-ng -v 음성 "텍스트"` : 특정 음성(voice)으로 출력
- `espeak-ng -s 속도 -p 음높이 "텍스트"` : 속도(speed) (기본값: 175)와 음높이(pitch) (기본값: 50) 설정
- `espeak-ng -w 경로/대상/출력파일.wav "텍스트"` : 음성을 직접 재생하지 않고 wAV 파일로 저장
- `espeak-ng --voices` : 사용 가능한 모든 음성 목록 출력

## Links
- More information: https://github.com/espeak-ng/espeak-ng/blob/master/src/espeak-ng.1.ronn
