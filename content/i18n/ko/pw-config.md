# pw-config

PipeWire 서버와 클라이언트에서 사용될 설정 경로와 섹션 나열.

## Usage
- `pw-config` : 사용될 모든 설정 파일 나열
- `pw-config -n pipewire-pulse.conf` : pipeWire PulseAudio 서버에서 사용될 모든 설정 파일 나열
- `pw-config -n pipewire-pulse.conf list` : pipeWire PulseAudio 서버에서 사용되는 모든 설정 섹션 나열
- `pw-config -n jack.conf list context.properties` : JACK 클라이언트에서 사용되는 `context.properties` 조각 나열
- `pw-config -n jack.conf merge context.properties` : JACK 클라이언트에서 사용되는 병합된 `context.properties` 나열
- `pw-config -n pipewire.conf -r merge context.modules` : pipeWire 서버에서 사용되는 병합된 `context.modules` 나열 및 reformat
- `pw-config -h` : 도움말 표시

## Links
- More information: https://docs.pipewire.org/page_man_pw-config_1.html
