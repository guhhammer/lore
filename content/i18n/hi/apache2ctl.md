# apache2ctl

अपाचे HTTP वेब सर्वर का प्रबंधन करें। यह कमांड डेबियन आधारित ओएस के साथ आता है, आरएचईएल आधारित ओएस के लिए `httpd` देखें।

## Usage
- `sudo apache2ctl start` : अपाचे डेमॉन प्रारंभ करें. यदि संदेश पहले से चल रहा हो तो उसे फेंकें
- `sudo apache2ctl stop` : अपाचे डेमॉन बंद करो
- `sudo apache2ctl restart` : अपाचे डेमॉन को पुनरारंभ करें
- `sudo apache2ctl -t` : कॉन्फ़िगरेशन फ़ाइल के सिंटैक्स का परीक्षण करें
- `sudo apache2ctl -M` : लोड किए गए मॉड्यूल की सूची बनाएं

## Links
- More information: https://manned.org/apache2ctl
