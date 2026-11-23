# btrfs

लिनक्स के लिए कॉपी-ऑन-राइट (COW) सिद्धांत पर आधारित एक फ़ाइल सिस्टम। कुछ सब-कमांड जैसे `btrfs device`, उनका खुद का उपयोग दस्तावेज़न है।

## Usage
- `sudo btrfs su c उप-वॉल्यूम/का/पथ` : उप-वॉल्यूम बनाएं
- `sudo btrfs su l माउंट_बिंदु/का/पथ` : उप-वॉल्यूमों की सूची दिखाएं
- `sudo btrfs f df माउंट_बिंदु/का/पथ` : स्थान उपयोग सूचना दिखाएं
- `sudo btrfs qu e उप-वॉल्यूम/का/पथ` : कोटा सक्षम करें
- `sudo btrfs qg s उप-वॉल्यूम/का/पथ` : कोटा दिखाएं

## Links
- More information: https://btrfs.readthedocs.io/en/latest/btrfs.html
