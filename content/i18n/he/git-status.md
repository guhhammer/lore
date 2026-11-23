# git status

הציגו את השינויים בקבצים במאגר Git. מפרט קבצים ששונו, נוספו או נמחקו בהשוואה ל-commit הנוכחי.

## Usage
- `git status` : הציגו קבצים שאינם במעקב וקבצים ששונו וטרם נוספו ל-commit
- `git status -s` : הציגו את הפלט בפורמט מקוצר
- `git status -sb` : הציגו את הפלט בפורמט מקוצר יחד עם מידע על הענף
- `git status -v` : הציגו את השינויים בקבצים שהוכנו ל-commit (כמו `git diff --cached`)
- `git status -vv` : הציגו את השינויים בכל הקבצים שבמעקב (כמו `git diff HEAD`)
- `git status --show-stash` : הציגו את מספר הרשומות השמורות כעת ב-stash
- `git status -uno` : הסתירו מהפלט קבצים שאינם במעקב

## Links
- More information: https://git-scm.com/docs/git-status
