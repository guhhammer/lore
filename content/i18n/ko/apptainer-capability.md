# apptainer capability

사용자 및 그룹의 Linux capability를 관리하는 도구.

## Usage
- `apptainer capability avail` : 사용 가능한 모든 Linux capability 목록 출력
- `apptainer capability avail cap_chown,cap_net_raw,...` : 특정 capability의 설명 출력
- `apptainer capability list` : 모든 사용자 및 그룹의 capability 목록 출력
- `apptainer capability list 사용자명|그룹명` : 모든 사용자 및 그룹의 capability 목록 출력
- `sudo apptainer capability add -u 사용자명 cap_net_raw,cap_chown,...` : 사용자에게 capability 추가
- `sudo apptainer capability add -g 그룹명 cap_net_raw,cap_chown,...` : 그룹에 capability 추가
- `sudo apptainer capability drop -u 사용자명 cap_net_raw,cap_chown,...` : 사용자에게서 capability 제거
- `sudo apptainer capability drop -u 사용자명 all` : 사용자에게서 모든 capability 제거

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_capability.html
