# gpasswd

إدارة `/etc/group` و `/etc/gshadow`.

## Usage
- `sudo gpasswd -A user1,user2 group` : عرّف مديرين المجموعة المسماة
- `sudo gpasswd -M user1,user2 group` : عين أعضاء المجموعة المسماة
- `gpasswd group` : إنشئ رقم سري للمجموعة المسماة
- `gpasswd -a user group` : أضف عضو إلي المجموعة المسماة
- `gpasswd -d user group` : إحذف عضو من المجموعة المسماة

## Links
- More information: https://manned.org/gpasswd
