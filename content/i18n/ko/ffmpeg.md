# ffmpeg

비디오 변환 도구.

## Usage
- `ffmpeg -i 경로/대상/비디오.mp4 -vn 경로/대상/소리.mp3` : 비디오에서 사운드를 추출하여 MP3로 저장
- `ffmpeg -i 경로/대상/입력_소리.flac -ar 44100 -sample_fmt s16 경로/대상/출력_소리.wav` : FLAC 파일을 Red Book CD 형식 (44100Hz, 16bit)으로 트랜스코딩
- `ffmpeg -i 경로/대상/비디오.mp4 -vf 'scale=-1:1000' -r 15 path/to/output.gif` : 비디오를 GIF로 저장하고, 높이를 1000px로 조정하고 프레임 속도를 15로 설정
- `ffmpeg -f image2 -i 경로/대상/프레임_%d.jpg video.mpg|video.gif` : 번호가 매겨진 이미지 (`frame_1.jpg`, `frame_2.jpg`, etc) 를 비디오 또는 GIF로 결합
- `ffmpeg -i 경로/대상/입력_비디오.mp4 -ss mm:ss -to mm2:ss2 -c copy 경로/대상/출력_비디오.mp4` : 주어진 시작 시간 mm:ss부터 종료 시간 mm2:ss2까지 비디오를 편집 (끝까지 다듬으려면 -to 플래그 생략)
- `ffmpeg -i 경로/대상/입력_비디오.avi -c:a aac -b:a 128k -c:v libx264 -crf 23 경로/대상/출력_비디오.mp4` : AVI 비디오를 MP4로 변환. AAC 오디오 @ 128kbit, h264 Video @ CRF 23
- `ffmpeg -i 경로/대상/입력_비디오.mkv -c copy 경로/대상/출력_비디오.mp4` : 오디오 또는 비디오 스트림을 다시 인코딩하지 않고 MKV 비디오를 MP4로 리먹싱
- `ffmpeg -i 경로/대상/입력_비디오.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus -vbr on -threads 스레드_수 경로/대상/출력_비디오.webm` : MP4 비디오를 VP9 코덱으로 변환. 최상의 품질을 위해서는, CRF 값(권장 범위 15-35)을 사용하고 -b:v는 0이어야 함

## Links
- More information: https://ffmpeg.org/ffmpeg.html#Options
