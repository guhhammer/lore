# bpftrace

Linux eBPF를 위한 고급 추적 언어.

## Usage
- `sudo bpftrace -l` : 사용 가능한 모든 프로브 나열
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'` : 원라이너 프로그램 실행 (예: 프로그램별 시스템 호출 수)
- `sudo bpftrace 경로/대상/파일` : 파일에서 프로그램 실행
- `sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter /pid == 123/ { @[comm] = count(); }'` : PID로 프로그램 추적
- `sudo bpftrace -d -e '한_줄_프로그램'` : 드라이런을 수행하고 eBPF 형식으로 출력 표시
- `bpftrace -V` : 버전 표시

## Links
- More information: https://github.com/bpftrace/bpftrace/blob/master/man/adoc/bpftrace.adoc
