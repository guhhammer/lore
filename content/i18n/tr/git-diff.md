# git diff

İzlenen dosyalara değişiklikleri göster.

## Usage
- `git diff` : sahnelenmemiş, commit'lenmemiş değişiklikleri göster
- `git diff HEAD` : sahnelenmiş olanlar da dahil olmak üzere tüm commit'lenmemiş değişiklikleri göster
- `git diff --staged` : yalnızca sahnelenmiş (eklenmiş ancak commit'lenmemiş) değişiklikleri göster
- `git diff 'HEAD@{3 months|weeks|days|hours|seconds ago}'` : belirtilen bir tarihten itibaren yapılmış tüm commit'lerdeki değişiklikleri göster (bir tarih ifadesi, örn. "1 week 2 days" ya da bir ISO tarihi)
- `git diff --stat commit` : değişen dosyalar, histogram ve toplam eklenen/silinen satır sayısı gibi diff istatistiklerini göster
- `git diff --summary commit` : belirtilen bir commit'ten itibaren yapılmış dosya oluşturma, yeniden adlandırma ve mod değişim işlemlerini göster
- `git diff dal_1..dal_2 örnek/dosya` : tek bir dosyayı iki dal veya commit arasında karşılaştır
- `git diff dal:örnek/dosya2 örnek/dosya` : mevcut daldaki farklı dosyaları başka bir daldakilerle karşılaştır

## Links
- More information: https://git-scm.com/docs/git-diff
