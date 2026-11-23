# vgchange

논리 볼륨 관리자(LVM) 볼륨 그룹의 속성을 변경합니다.

## Usage
- `sudo vgchange -a y|n` : 모든 볼륨 그룹의 논리 볼륨 활성화 상태 변경
- `sudo vgchange -a y|n 볼륨_그룹` : 지정된 볼륨 그룹의 논리 볼륨 활성화 상태 변경 (`vgscan`으로 확인 가능)

## Links
- More information: https://manned.org/vgchange
