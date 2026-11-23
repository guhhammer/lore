# choco source

चॉकलेटी वाले पैकेजों के लिए स्रोत प्रबंधित करें।

## Usage
- `choco source list` : वर्तमान में उपलब्ध स्रोतों की सूची बनाएं
- `choco source add -n नाम -s यूआरएल` : एक नया पैकेज स्रोत जोड़ें
- `choco source add -n नाम -s यूआरएल -u उपयोगकर्ता_नाम -p पासवर्ड` : क्रेडेंशियल्स के साथ एक नया पैकेज स्रोत जोड़ें
- `choco source add -n नाम -s यूआरएल --cert प्रमाणपत्र_फ़ाइल\का\पथ` : क्लाइंट प्रमाणपत्र के साथ एक नया पैकेज स्रोत जोड़ें
- `choco source enable -n नाम` : पैकेज स्रोत सक्षम करें
- `choco source disable -n नाम` : पैकेज स्रोत को अक्षम करें
- `choco source remove -n नाम` : पैकेज स्रोत हटाएँ

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
