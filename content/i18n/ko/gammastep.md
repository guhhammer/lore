# gammastep

하루 중 시간에 따라 화면의 색온도를 조정.

## Usage
- `gammastep -t 5700:3600` : 낮(예: 5700k)과 밤(예: 3600k)에 특정 온도(temperature)로 Gammastep은 켜기
- `gammastep -l latitude:longitude` : 수동으로 지정한 사용자 정의 위치(location)로 Gammastep을 켜기
- `gammastep -b 0.7:0.4` : 낮(예: 70%)과 밤(예: 40%)의 특정 화면 밝기(brightness)로 최소 밝기 10% 및 최대 밝기 100%로 Gammastep을 켜기
- `gammastep -g red:green:blue` : 사용자 정의 gamma 레벨 (0과 1 사이)로 Gammastep을 켜기
- `gammastep -O 온도` : 지속적으로 변하지 않는 색온도(cOnstant) Gammastep을 켜기
- `gammastep -x` : gammastep에 의해 적용된 온도 조정 재설정

## Links
- More information: https://manned.org/gammastep
