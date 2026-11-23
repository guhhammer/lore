# pulseaudio

PulseAudio 사운드 시스템 데몬 및 관리자.

## Usage
- `pulseaudio --check` : pulseAudio가 실행 중인지 확인 (0이 아닌 종료 코드는 실행 중이 아님을 의미)
- `pulseaudio --start` : 백그라운드에서 PulseAudio 데몬 시작
- `pulseaudio -k` : 실행 중인 PulseAudio 데몬 종료
- `pulseaudio --dump-modules` : 사용 가능한 모듈 나열
- `pulseaudio -L "모듈_이름 인수"` : 현재 실행 중인 데몬에 모듈과 지정된 인수를 로드

## Links
- More information: https://manned.org/pulseaudio
