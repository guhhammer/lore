# handbrakecli

HandBrake 비디오 변환 및 DVD 리핑 도구에 대한 명령줄 인터페이스.

## Usage
- `handbrakecli -i 입력.avi -o 출력.mkv -e x264 -q 20 -B 160` : 비디오 파일을 MKV(AAC 160kbit 오디오 및 x264 CRF20 비디오)로 변환
- `handbrakecli -i 입력.mp4 -o 출력.mp4 -w 320 -l 240` : 비디오 파일 크기를 320x240으로 조정
- `handbrakecli -z` : 사용 가능한 사전 설정 목록
- `handbrakecli -Z "Android" -i 입력.ext -o 출력.mp4` : android 사전 설정을 사용하여 AVI 비디오를 MP4로 변환
- `handbrakecli -i /dev/sr0 -t 0` : DVD 내용을 출력하고 그 과정에서 CSS 키를 가져옴
- `handbrakecli -i /dev/sr0 -t 1 -o out.mkv -f av_mkv -e x264 -s 1,4,5 -a 1,2 -E copy -q 23` : 지정된 장치에서 DVD의 첫 번째 트랙을 추출. 오디오트랙과 자막 언어는 목록으로 지정됨

## Links
- More information: https://handbrake.fr/docs/en/latest/cli/command-line-reference.html
