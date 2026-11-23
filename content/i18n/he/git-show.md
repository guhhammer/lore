# git show

הצגת אובייקטים מסוגים שונים ב-Git (כגון commits ותגיות).

## Usage
- `git show` : הצגת מידע על ה-commit האחרון (מזהה, הודעה, שינויים ופרטים נוספים)
- `git show commit|tag|branch` : הצגת מידע על commit, תגית או ענף מסוימים (למשל `HEAD` עבור ה-commit האחרון)
- `git show --summary commit` : הצגת רשימת הקבצים שנוספו, ששמם שונה או שנמחקו בלבד
- `git show -w` : התעלמות מתווי רווח בעת השוואת שורות
- `git show --oneline -s commit` : הצגת הודעת commit בשורה אחת, ללא הצגת ההבדלים
- `git show --stat commit` : הצגת נתונים סטטיסטיים על הקבצים שהשתנו (כגון מספר השורות שנוספו ונמחקו)
- `git show --name-only commit` : הצגת רשימה פשוטה של כל הקבצים שהשתנו ב-commit (שונו, נוספו או נמחקו)
- `git show revision:path/to/file` : הצגת תוכן קובץ כפי שהיה בגרסה מסוימת (למשל ענף, תגית או commit)

## Links
- More information: https://git-scm.com/docs/git-show
