# wpctl

WirePlumber를 관리하는 도구로, PipeWire의 세션 및 정책 관리자를 관리합니다.

## Usage
- `wpctl status` : wirePlumber가 관리하는 모든 객체 나열
- `wpctl inspect id` : 객체의 모든 속성 출력
- `wpctl set-default id` : 객체를 해당 그룹의 기본값으로 설정
- `wpctl get-volume id` : 싱크의 볼륨 가져오기
- `wpctl set-volume id n%` : 싱크의 볼륨을 `n` 퍼센트로 설정
- `wpctl set-volume id n%+|-` : 싱크의 볼륨을 `n` 퍼센트만큼 증가/감소
- `wpctl set-mute id 1|0|toggle` : 싱크 음소거/음소거 해제 설정 (1은 음소거, 0은 음소거 해제)

## Links
- More information: https://pipewire.pages.freedesktop.org/wireplumber/
