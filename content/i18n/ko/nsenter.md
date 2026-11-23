# nsenter

실행 중인 프로세스의 네임스페이스에서 새로운 명령을 실행합니다. Docker 이미지나 chroot 감옥에 특히 유용합니다.

## Usage
- `nsenter -t pid -a 명령 명령_인수` : 특정 프로세스와 동일한 네임스페이스를 사용하여 명령 실행
- `nsenter -t pid --mount|uts|ipc|net|pid|user|cgroup 명령 명령_인수` : 특정 프로세스의 마운트|UTS|IPC|네트워크|PID|사용자|cgroup|시간 네임스페이스에서 명령 실행
- `nsenter -t pid -u -T -i -- 명령 명령_인수` : 특정 프로세스의 UTS, 시간, IPC 네임스페이스에서 명령 실행
- `nsenter -n/proc/pid/ns/net -- 명령 명령_인수` : procfs를 참조하여 특정 프로세스의 네임스페이스에서 명령 실행

## Links
- More information: https://manned.org/nsenter
