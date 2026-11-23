# mssqlclient.py

Microsoft SQL Server 인스턴스에 연결하고 쿼리를 실행. Impacket 도구 모음의 일부.

## Usage
- `mssqlclient.py -windows-auth 도메인/사용자명:비밀번호@대상_서버` : windows 인증을 사용하여 MSSQL 서버에 연결
- `mssqlclient.py 사용자명:비밀번호@대상_서버` : SQL 서버 인증을 사용하여 연결
- `mssqlclient.py 도메인/사용자명@대상_서버 -hashes LM_Hash:NT_Hash` : 해시 기반 인증을 사용해 연결
- `mssqlclient.py -k 도메인/사용자명@대상_서버` : kerberos 인증을 사용하여 연결 (유효한 티켓 필요)
- `mssqlclient.py 사용자명:비밀번호@대상_서버 -query "SELECT user_name();"` : 연결 후 특정 SQL 명령 실행
- `mssqlclient.py 사용자명:비밀번호@대상_서버 -file 경로/대상/sql_파일.sql` : 파일에 있는 여러 SQL 명령 실행
- `mssqlclient.py 사용자명:비밀번호@대상_서버 -db 데이터베이스_이름` : 특정 데이터베이스에 연결 (기본값: `None`)
- `mssqlclient.py 사용자명:비밀번호@대상_서버 -show` : 실행 전에 SQL 쿼리 표시

## Links
- More information: https://github.com/fortra/impacket
