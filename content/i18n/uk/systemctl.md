# systemctl

Керуйте системою systemd і диспетчером служб.

## Usage
- `systemctl status` : відобразити всі запущені служби
- `systemctl --failed` : відобразити список служб, які не запустилися через помилки
- `systemctl start|stop|restart|reload служба` : запуск/зупинка/перезапуск/перезавантаження служби
- `systemctl status служба` : показати статус служби
- `systemctl enable|disable служба` : увімкнути/вимкнути запуск служби під час завантаження
- `systemctl mask|unmask служба` : маскування/розкриття служби, щоб запобігти ввімкненню та ручній активації
- `systemctl daemon-reload` : перезавантажити systemd, шукати нові або змінені пристрої
- `systemctl is-enabled служба` : перевірити, чи ввімкнено службу до автозавантаження

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
