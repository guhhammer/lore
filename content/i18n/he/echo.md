# echo

הדפסת הארגומנטים שסופקו.

## Usage
- `echo "שלום עולם"` : הדפסת הודעת טקסט. הערה: המרכאות אופציונליות
- `echo "הנתיב שלי הוא $PATH"` : הדפסת הודעה עם משתני סביבה
- `echo -n "שלום עולם"` : הדפסת הודעה ללא ירידת שורה בסוף
- `echo "שלום עולם" >>קובץ.txt` : הוספת הודעה לסוף קובץ
- `echo -e "עמודה 1\tעמודה 2"` : הפעלת פירוש של תווי בקסלש (תווים מיוחדים)
- `echo $?` : הדפסת מצב היציאה של הפקודה האחרונה שהורצה (הערה: ב-Windows Command Prompt וב-PowerShell הפקודות המקבילות הן `echo %errorlevel%` ו-`$lastexitcode` בהתאמה)
- `echo "שלום עולם" | תוכנית` : העברת טקסט לתוכנית אחרת דרך `stdin`

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/echo-invocation.html
