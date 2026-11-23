# roll

사용자 정의 주사위 시퀀스를 굴립니다.

## Usage
- `roll 3d` : 3개의 6면체 주사위를 굴리고 결과를 합산
- `roll d8 + 3` : 1개의 8면체 주사위를 굴리고, 3을 더하여 결과 합산
- `roll 4d6h3` : 4개의 6면체 주사위를 굴리고 상위 3개의 결과를 유지하여 합산
- `roll --verbose 2{2d12}` : 12면체 주사위 2개를 2번 굴리고 각 결과를 표시
- `roll "2d20>10"` : 2개의 20면체 주사위를 굴려 결과가 10보다 클 때까지 반복
- `roll --sum-series 3{2d5}` : 5면체 주사위 2개를 3번 굴리고 총합을 표시

## Links
- More information: https://manned.org/roll
