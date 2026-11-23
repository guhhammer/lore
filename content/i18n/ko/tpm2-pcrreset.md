# tpm2 pcrreset

하나 이상의 PCR 뱅크를 초기화. 참고: 운영체제의 locality에서는, PCR 16과 23만 초기화할 수 있음.

## Usage
- `tpm2 pcrreset 23` : PCR 23 뱅크 초기화
- `tpm2 pcrreset 16 23` : PCR 16과 23 뱅크 초기화

## Links
- More information: https://manned.org/tpm2_pcrreset
