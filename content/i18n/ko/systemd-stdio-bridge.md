# systemd-stdio-bridge

`stdin`/`stdout`와 D-Bus 사이에 프록시를 구현합니다. 참고: 시작 시 `stdin`/`stdout`을 통해 열린 연결을 수신하도록 되어 있으며, 지정된 버스로 새로운 연결을 생성합니다.

## Usage
- `systemd-stdio-bridge` : `stdin`/`stdout`을 로컬 시스템 버스로 전달
- `systemd-stdio-bridge --사용자_명` : `stdin`/`stdout`을 특정 사용자의 D-Bus로 전달
- `systemd-stdio-bridge -M mycontainer` : 특정 컨테이너 내에서 `stdin`/`stdout`을 로컬 시스템 버스로 전달
- `systemd-stdio-bridge -p unix:path=/custom/dbus/socket` : 사용자 지정 D-Bus 주소로 `stdin`/`stdout` 전달
