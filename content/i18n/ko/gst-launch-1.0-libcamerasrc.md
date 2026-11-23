# gst-launch-1.0 libcamerasrc

libcamera 장치로부터 프레임 읽기.

## Usage
- `gst-launch-1.0 libcamerasrc ! videoconvert ! autovideosink` : 비디오를 창에서 출력
- `gst-launch-1.0 libcamerasrc camera-name=카메라_이름 ! 비디오_포맷_변환_요소 ! 자동_선택된_비디오_출력` : 특정 장치를 지정하여 비디오 입력 읽기

## Links
- More information: https://libcamera.org/getting-started.html#using-gstreamer-plugin
