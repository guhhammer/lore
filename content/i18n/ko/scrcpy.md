# scrcpy

Android 기기를 데스크톱에서 표시하고 제어.

## Usage
- `scrcpy` : 연결된 기기의 화면 미러링
- `scrcpy --serial 0123456789abcdef|192.168.0.1:5555` : ID 또는 IP 주소를 기반으로 특정 기기의 화면 미러링 (`adb devices` 명령어로 확인 가능)
- `scrcpy --fullscreen` : 전체 화면 모드로 시작
- `scrcpy --rotation 0|1|2|3` : 화면 회전. 각 증가 값은 반시계 방향으로 90도 회전을 추가
- `scrcpy --show-touches` : 물리적 기기에서 터치 표시
- `scrcpy --record 경로/대상/파일.mp4` : 화면 기록
- `scrcpy --push-target 경로/대상/폴더` : 드래그 앤 드롭으로 파일을 기기에 전송할 대상 디렉터리 지정 (APK가 아님)

## Links
- More information: https://github.com/Genymobile/scrcpy
