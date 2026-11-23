# mimikatz dpapi

Windows Data Protection API (DPAPI)와 상호작용.

## Usage
- `mimikatz "dpapi::masterkey /list"` : 마스터 키 목록 표시
- `mimikatz "dpapi::blob /in:blob_file.bin"` : DPAPI blob 복호화
- `mimikatz "dpapi::chrome /in:Login Data"` : DPAPI를 사용하여 Chrome 자격 증명 가져오기

## Links
- More information: https://github.com/gentilkiwi/mimikatz
