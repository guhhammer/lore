# liquidctl

수랭 쿨러 제어.

## Usage
- `liquidctl list` : 사용 가능한 장치 목록 표시
- `sudo liquidctl initialize all` : 지원되는 모든 장치 초기화
- `liquidctl status` : 사용 가능한 수랭 쿨러 상태 출력
- `liquidctl -m 문자열 set fan|pump|... speed 50` : 제품명에 특정 문자열이 포함된 장치를 선택해, 팬 또는 펌프 속도를 50%으로 설정
- `liquidctl set fan|pump|... speed 20 0 50 50 70 100` : 온도에 따라 점진적으로 증가하는 팬 속도 곡선 설정 (20°C는 0%, 50°C는 50%, 70°C는 100%)
- `liquidctl set fan|pump|... color fixed|fading|rainbow|... 00ff00` : 수랭 쿨러 RGB 조명 설정 (지원 모드는 모델에 따라 다름)

## Links
- More information: https://github.com/liquidctl/liquidctl
