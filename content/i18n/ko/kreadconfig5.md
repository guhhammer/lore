# kreadconfig5

KDE Plasma의 KConfig 항목 읽기.

## Usage
- `kreadconfig5 --group 그룹_이름 --key 키_이름` : 전역 설정에서 키 읽기
- `kreadconfig5 --file 경로/대상/파일 --group 그룹_이름 --key 키_이름` : 특정 설정 파일에서 키 읽기
- `kreadconfig5 --file startkderc --group General --key systemdBoot` : systemd가 Plasma 세션을 시작하는지 확인

## Links
- More information: https://userbase.kde.org/KDE_System_Administration/Configuration_Files
