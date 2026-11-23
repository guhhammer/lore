# numfmt

숫자를 사람이 읽기 쉬운 문자열로 변환하거나 그 반대로 변환.

## Usage
- `numfmt --from si 1.5K` : 1.5K(SI 단위)를 1500으로 변환
- `ls -l | numfmt --header=1 --field 5 --to iec` : 5번째 필드(1부터 시작)를 IEC 단위로 변환하되 헤더는 변환하지 않음
- `du -s * | numfmt --to iec --format "%-5f"` : IEC 단위로 변환하고, 5자리를 채워 왼쪽 정렬
