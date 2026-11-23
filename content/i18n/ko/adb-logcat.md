# adb logcat

시스템 메시지의 로그 덤프.

## Usage
- `adb logcat` : 시스템 로그 표시
- `adb logcat -e 정규_표현식` : 정규 표현식(expression)과 일치하는 행 표시
- `adb logcat 태그:모드 *:S` : 특정 모드(상세 (Verbose), 디버그(Debug), 정보(Info), 경고(Warning), 에러(Error), 치명적 오류(Fatal), 무음(Silent))에서 태그에 대한 로그를 표시하고 다른 태그를 필터링
- `adb logcat ReactNative:V ReactNativeJS:V *:S` : 다른 태그를 무음으로(Silencing), 상세 (Verbose) 모드에서 React Native 애플리케이션에 대한 로그를 표시
- `adb logcat *:W` : 우선순위 수준이 경고(Warning) 이상인 모든 태그에 대한 로그 표시
- `adb logcat --pid pid` : 특정 PID에 대한 로그 표시
- `adb logcat --pid $(adb shell pidof -s 패키지)` : 특정 패키지의 프로세스에 대한 로그 표시
- `adb logcat -v color` : 로그 색상 지정(보통 필터와 함께 사용)

## Links
- More information: https://developer.android.com/tools/logcat
