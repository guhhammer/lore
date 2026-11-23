# git add

يضيف الملفات المعدلة إلى منطقة التجميع (staging area).

## Usage
- `git add path/to/file` : إضافة ملف إلى منطقة التجميع (staging area)
- `git add -A` : إضافة جميع الملفات (المتتبَّعة وغير المتتبَّعة Tracked & Untracked)
- `git add .` : إضافة جميع الملفات بشكل متكرر (recursively) بدءًا من المُجَلَّد الحالي
- `git add -u` : إضافة الملفات المتتبَّعة (Tracked) فقط
- `git add -f path/to/file` : إضافة الملفات المتجاهلة (Ignored) أيضًا
- `git add -p` : إضافة أجزاء من الملفات بشكل تفاعلي (Interactive)
- `git add -p path/to/file` : إضافة أجزاء من ملف معين بشكل تفاعلي
- `git add -i` : إضافة ملف بشكل تفاعلي

## Links
- More information: https://git-scm.com/docs/git-add
