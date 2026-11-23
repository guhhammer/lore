# gst-launch-1.0 videoscale

비디오 프레임 크기 조정.

## Usage
- `gst-launch-1.0 입력 ! videoscale ! 출력` : 입력과 출력 capability에 맞게 비디오 크기 조정
- `gst-launch-1.0 입력 ! videoscale ! video/x-raw,width=너비,height=높이 ! 출력` : 지정한 해상도로 비디오 크기 조정
