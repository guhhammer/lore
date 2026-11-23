# systemctl

systemd सिस्टम और सेवा प्रबंधक को नियंत्रित करें।

## Usage
- `systemctl status` : सभी चल रही सेवाएँ दिखाएं
- `systemctl --failed` : विफल इकाइयों की सूची
- `systemctl start|stop|restart|reload इकाई` : सेवा को चालना/रोकना/पुनरारंभ करना/रीलोड करना
- `systemctl status इकाई` : एक इकाई की स्थिति दिखाएं
- `systemctl enable|disable इकाई` : एक इकाई को बूटअप पर स्वचलित रूप से चालाने/रोकने के लिए सक्षम/अक्षम करें
- `systemctl mask|unmask इकाई` : एक इकाई को सक्षम/अक्षम करने और मैन्युअल सक्रियण से रोकने/हटाने के लिए मास्क/अनमास्क करें
- `systemctl daemon-reload` : systemd को पुनः लोड करें, नई या बदली गई इकाइयों के लिए स्कैन करें
- `systemctl is-enabled इकाई` : क्या किसी इकाई को सक्षम किया गया है, यह जाँचें

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
