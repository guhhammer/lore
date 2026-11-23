# systemctl mount-image

이미지 파일을 유닛의 마운트 네임스페이스에 마운트. `RootImage=`, `PrivateMounts=` 등 마운트 네임스페이스를 사용하는 유닛에서만 지원됨.

## Usage
- `systemctl mount-image 유닛 /경로/대상/이미지 /경로/대상/디렉터리_내부_유닛` : 유닛 내부의 특정 경로에 이미지 마운트
- `systemctl mount-image 유닛 /경로/대상/이미지 /경로/대상/디렉터리_내부_유닛 root:ro,nosuid` : 이미지의 `root` 파티션을 읽기 전용 및 no setuid 옵션으로 마운트
- `systemctl mount-image --mkdir 유닛 /경로/대상/이미지 /경로/대상/디렉터리_내부_유닛` : 마운트 전에 대상 디렉터리 생성
- `systemctl mount-image --read-only 유닛 /경로/대상/이미지 /경로/대상/디렉터리_내부_유닛` : 이미지를 읽기 전용으로 마운트
