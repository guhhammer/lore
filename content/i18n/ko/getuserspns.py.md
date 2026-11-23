# getuserspns.py

Active Directory 사용자 계정에 연결된 Service Principal Names (SPN) 조회. Impacket 도구 모음의 일부.

## Usage
- `GetUserSPNs.py 도메인/사용자명:비밀번호 -dc-ip 도메인_컨트롤러_ip` : SPN이 설정된 사용자 계정을 열거하고 Kerberos TGS 티켓 요청
- `GetUserSPNs.py 도메인/사용자명 -hashes LM_Hash:NT_Hash -dc-ip 도메인_컨트롤러_ip` : pass-the-hash 인증 사용
- `GetUserSPNs.py 도메인/사용자명:비밀번호 -dc-ip 도메인_컨트롤러_ip -outputfile 경로/대상/출력_파일` : 결과를 파일로 저장
- `GetUserSPNs.py 도메인/사용자명:비밀번호 -dc-ip 도메인_컨트롤러_ip -request` : TGS 티켓만 요청
- `GetUserSPNs.py 도메인/사용자명 -dc-ip 도메인_컨트롤러_ip -hashes LM_Hash:NT_Hash -request` : pass-the-hash 인증을 사용하여 TGS 티켓만 요청

## Links
- More information: https://github.com/fortra/impacket
