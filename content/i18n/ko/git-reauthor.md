# git reauthor

작성자 신원에 대한 세부 정보를 변경합니다. 이 명령은 Git 기록을 재작성하므로 다음 푸시 시 `--force`가 필요합니다. `git-extras`의 일부입니다.

## Usage
- `git reauthor -o old@example.com -e new@example.com -n "name"` : git 저장소 전체에서 작성자의 이메일과 이름 변경
- `git reauthor -o old@example.com -c` : git 설정에 정의된 이메일과 이름으로 변경
- `git reauthor -a -e name@example.com -n name` : 원래 작성자와 관계없이 모든 커밋의 이메일과 이름 변경

## Links
- More information: https://github.com/tj/git-extras/blob/main/Commands.md#git-reauthor
