# msfconsole

Metasploit 프레임워크의 콘솔.

## Usage
- `msfconsole` : 콘솔 시작
- `msfconsole --quiet` : 배너 없이 조용히 콘솔 시작
- `msfconsole --no-database` : 데이터베이스 지원을 비활성화하고 시작
- `msfconsole --execute-command "use auxiliary/server/capture/ftp; set SRVHOST 0.0.0.0; set SRVPORT 21; run"` : 콘솔 명령 실행 (참고: 여러 명령을 전달하려면 `;` 사용)
- `msfconsole --version` : 버전 표시

## Links
- More information: https://docs.rapid7.com/metasploit/msf-overview/
