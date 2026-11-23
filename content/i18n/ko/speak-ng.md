# speak-ng

다국어를 지원하는 소프트웨어 음성 합성기.

## Usage
- `speak-ng "텍스트"` : 문구를 음성으로 읽기
- `echo "텍스트" | speak-ng` : `stdin`의 텍스트를 음성으로 읽기
- `speak-ng -f 경로/대상/파일` : 파일(file)의 내용을 음성으로 읽기
- `speak-ng -v 음성 "텍스트"` : 지정한 음성(voice)을 사용하여 읽기
- `speak-ng -s 속도 -p 음높이 "텍스트"` : 지정한 속도(speed)와 음높이(pitch)로 읽기 (기본값: 속도 175, 음높이 50)
- `speak-ng -w 경로/대상/출력파일.wav "텍스트"` : 직접 재생하지 않고 wAV 파일로 출력
- `speak-ng --voices` : 사용 가능한 모든 음성 목록 표시

## Links
- More information: https://github.com/espeak-ng/espeak-ng/blob/master/src/speak-ng.1.ronn
