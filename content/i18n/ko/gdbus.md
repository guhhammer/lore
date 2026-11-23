# gdbus

D-Bus 객체와 상호작용. GLib의 일부.

## Usage
- `gdbus list-names --session` : 세션 버스의 모든 이름 목록 표시
- `gdbus list-names --system` : 시스템 버스의 모든 이름 목록 표시
- `gdbus introspect --session --dest 목적지_버스_이름 --object-path /경로/대상/객체` : 객체를 조사해 인터페이스와 메서드 표시
- `gdbus call --session --dest 목적지_버스_이름 --object-path /경로/대상/객체 --method 인터페이스.메소드_이름 인자1 인자2 ...` : 객체의 메서드를 인수로 전달하여 호출
- `gdbus emit --session --object-path /경로/대상/객체 --signal 인터페이스.시그널_이름 인자1 인자2 ...` : 객체에서 인수를 포함한 시그널 전송
- `gdbus monitor --session` : 세션 버스의 모든 메시지 모니터링

## Links
- More information: https://manned.org/gdbus
