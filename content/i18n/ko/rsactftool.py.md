# rsactftool.py

CTF 문제를 위한 RSA 공격 도구 - 취약 한 공개 키에서 개인 키를 복구하거나 데이터를 복호화함.

## Usage
- `RsaCtfTool.py --publickey 경로/대상/키.pub --private` : 공개 키 파일에서 개인키 복구
- `RsaCtfTool.py --publickey 경로/대상/키.pub --decryptfile path/to/ciphered_file` : 공개 키를 사용하여 파일 복호화
- `RsaCtfTool.py --publickey 경로/대상/키.pub --decrypt "암호문"` : 지정한 암호문 문자열 복호화
- `RsaCtfTool.py --dumpkey --key 경로/대상/키.pub` : 키 파일에서 RSA 키 구성 요소 (예: 모듈러스, 지수) 출력
- `RsaCtfTool.py --publickey 경로/대상/키.pub --private --attack fermat` : 지정한 공격 방식 (예: Fermat 인수분해) 으로 개인 키 복구
- `RsaCtfTool.py --createpub -n 모듈러스 -e 지수` : 모듈러스(n)와 지수(e)를 사용하여 공개 키 생성
- `RsaCtfTool.py --publickey 경로/대상/키.pub --private --attack all` : 사용 가능한 모든 공격 방식을 시도하여 개인 키 복구

## Links
- More information: https://github.com/RsaCtfTool/RsaCtfTool#usage
