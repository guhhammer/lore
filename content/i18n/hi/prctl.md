# prctl

चल रहे प्रक्रियाओं, कार्यों और परियोजनाओं के संसाधन नियंत्रण प्राप्त करें या सेट करें।

## Usage
- `prctl pid` : प्रक्रिया सीमाओं और अनुमतियों की जांच करें
- `prctl -P pid` : मशीन पार्सेबल प्रारूप में प्रक्रिया सीमाओं और अनुमतियों की जांच करें
- `prctl -n process.max-file-descriptor pid` : चल रही प्रक्रिया के लिए विशिष्ट सीमा प्राप्त करें

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
