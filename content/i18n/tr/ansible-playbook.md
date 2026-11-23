# ansible-playbook

Playbook'ta tanımlanan görevleri uzak makinelerde SSH üzerinden çalıştır.

## Usage
- `ansible-playbook playbook` : playbook'taki görevleri çalıştır
- `ansible-playbook playbook -i envanter_dosyası` : playbook'taki görevleri özel bir ana makine envanteriyle çalıştır
- `ansible-playbook playbook -e "değişken1=değer1 değişken2=değer2"` : playbook'taki görevleri komut satırında tanımlanan ek değişkenlerle çalıştır
- `ansible-playbook playbook -e "@değişkenler.json"` : playbook'taki görevleri bir JSON dosyasında tanımlanan ek değişkenlerle çalıştır
- `ansible-playbook playbook -t etiket1,etiket2` : playbook'ta yalnızca belirtilen etiketlere sahip görevleri çalıştır
- `ansible-playbook playbook --start-at-task görev_adı` : playbook'u belirli bir görevden başlayarak çalıştır
- `ansible-playbook playbook -C -D` : playbook'taki görevleri hiçbir değişiklik yapmadan deneme amacıyla çalıştır

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
