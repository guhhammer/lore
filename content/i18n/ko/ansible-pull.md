# ansible-pull

VCS 저장소에서 Ansible 플레이북을 가져와 로컬 호스트에서 실행.

## Usage
- `ansible-pull -U 저장소_url` : VCS에서 플레이북을 가져와 기본 `local.yml` playbook을 실행
- `ansible-pull -U 저장소_url playbook` : VCS에서 플레이북을 가져와 특정 플레이북을 실행
- `ansible-pull -U 저장소_url -C branch playbook` : 특정 지점의 VCS에서 플레이북을 가져와 특정 플레이북을 실행
- `ansible-pull -U 저장소_url -i hosts_file playbook` : VCS에서 플레이북을 가져오고, 호스트 파일을 지정하고 특정 플레이북을 실행

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
