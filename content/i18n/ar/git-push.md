# git push

دفع التعديلات إلى مستودع Git خارجي.

## Usage
- `git push` : دفع التعديلات المحلية في الفرع الحالي إلى نظيره في المستودع الخارجي
- `git push remote_name local_branch` : دفع التعديلات من فرع محلي معين إلى نظيره في المستودع الخارجي
- `git push -u remote_name local_branch` : دفع التعديلات من فرع محلي معين إلى نظيره في المستودع الخارجي، وتعيين الفرع الخارجي كهدف افتراضي لعمليات الدفع والسحب
- `git push remote_name local_branch:remote_branch` : دفع التعديلات من فرع محلي معين إلى فرع خارجي محدد
- `git push --all remote_name` : دفع جميع الفروع المحلية إلى نظائرها في المستودع الخارجي
- `git push remote_name -d remote_branch` : حذف فرع معين من المستودع الخارجي
- `git push --prune remote_name` : حذف الفروع البعيدة التي لا تمتلك نظيرًا محليًا بعد الآن
- `git push --tags` : نشر العلامات (tags) غير الموجودة في المستودع الخارجي

## Links
- More information: https://git-scm.com/docs/git-push
