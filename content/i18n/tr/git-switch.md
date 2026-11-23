# git switch

Git dalları arasında geçiş yap. Gir sürümü 2.23+ olmalıdır.

## Usage
- `git switch dal_ismi` : varolan bir dala geç
- `git switch -c dal_ismi` : yeni bir dal yarat ve ona geç
- `git switch -c dal_ismi commit` : varolan commit üzerine yeni bir dal yarat ve ona geç
- `git switch -` : önceki dala geç
- `git switch --recurse-submodules dal_ismi` : bir dala geç ve tüm alt modülleri uyum için güncelle
- `git switch -m dal_ismi` : bir dala geç ve mevcut dal ile commit'lenmeyen değişiklikleri bu dal ile birleştir

## Links
- More information: https://git-scm.com/docs/git-switch
