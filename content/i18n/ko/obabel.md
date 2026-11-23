# obabel

화학 관련 데이터를 변환합니다.

## Usage
- `obabel 경로/대상/파일.mol -O 경로/대상/출력_파일.xyz` : `.mol` 파일을 XYZ 좌표로 변환
- `obabel -:"SMILES" -O 경로/대상/출력_파일.png -xp 500` : SMILES 문자열을 500x500 그림으로 변환
- `obabel 경로/대상/파일.smi -O 경로/대상/출력_파일.mol --gen3D -m` : SMILES 문자열 파일을 개별 3D `.mol` 파일로 변환
- `obabel 경로/대상/파일1 경로/대상/파일2 ... -O 경로/대상/출력_파일.png` : 여러 입력을 하나의 그림으로 렌더링

## Links
- More information: https://open-babel.readthedocs.io/en/latest/Command-line_tools/babel.html
