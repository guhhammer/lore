# screencap

모바일 디스플레이의 스크린샷 찍기. 참고: 이 명령은 `adb shell`을 통해서만 사용할 수 있습니다.

## Usage
- `screencap 경로/대상/파일` : 스크린샷 찍기
- `screencap -p` : 화면을 캡처해 PNG 형식으로 표준 출력(`stdout`)에 표시
- `adb shell screencap -p >경로/대상/이미지.png` : `adb` 연결 통해 화면을 캡처하고 로컬 파일로 저장
- `screencap -h` : 도움말 표시

## Links
- More information: https://developer.android.com/tools/adb#screencap
