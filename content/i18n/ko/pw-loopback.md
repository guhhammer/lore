# pw-loopback

PipeWire에서 루프백 장치를 생성.

## Usage
- `pw-loopback` : 기본 루프백 동작으로 루프백 장치 생성
- `pw-loopback -m '[FL FR]' -i 'media.class=Audio/Sink'` : 스피커에 자동으로 연결되는 루프백 장치 생성
- `pw-loopback -m '[FL FR]' -o 'media.class=Audio/Source'` : 마이크에 자동으로 연결되는 루프백 장치 생성
- `pw-loopback -m '[FL FR]' -i 'media.class=Audio/Sink' -o 'media.class=Audio/Source'` : 자동으로 아무것에도 연결되지 않는 더미 루프백 장치 생성
- `pw-loopback -i 'media.class=Audio/Sink audio.position=[FL FR]' -o 'audio.position=[FR FL]'` : 스피커에 자동으로 연결되고 싱크와 소스 간 좌우 채널을 교환하는 루프백 장치 생성
- `pw-loopback -i 'audio.position=[FR FL]' -o 'media.class=Audio/Source audio.position=[FL FR]'` : 마이크에 자동으로 연결되고 싱크와 소스 간 좌우 채널을 교환하는 루프백 장치 생성

## Links
- More information: https://docs.pipewire.org/page_man_pw-loopback_1.html
