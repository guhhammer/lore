# svcs

चल रहे सेवाओं के बारे में जानकारी सूचीबद्ध करें।

## Usage
- `svcs` : सभी चल रही सेवाओं की सूची बनाएं
- `svcs -vx` : उन सेवाओं की सूची बनाएं जो चल नहीं रही हैं
- `svcs apache` : किसी सेवा के बारे में जानकारी सूचीबद्ध करें
- `svcs -L apache` : सेवा लॉग फ़ाइल के स्थान को दिखाएं
- `tail $(svcs -L apache)` : सेवा लॉग फ़ाइल के अंत को प्रदर्शित करें

## Links
- More information: https://www.unix.com/man-page/sunos/1/svcs
