# gst-launch-1.0 videotestsrc

테스트용 비디오 데이터 생성.

## Usage
- `gst-launch-1.0 videotestsrc ! 자동_선택된_비디오_출력` : SMPTE 컬러 바 비디오 생성
- `gst-launch-1.0 videotestsrc pattern=smpte|snow|green|ball|circular|... ! 자동_선택된_비디오_출력` : 생성할 테스트 비디오 패턴 지정

## Links
- More information: https://gstreamer.freedesktop.org/documentation/videotestsrc/index.html
