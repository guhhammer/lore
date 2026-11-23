# devfsadm

`/dev` के लिए प्रशासनिक आदेश। `/dev` नामस्थान को बनाए रखता है।

## Usage
- `devfsadm -c disk` : नए डिस्क के लिए स्कैन करें
- `devfsadm -C -v` : किसी भी लटके हुए /dev लिंक को साफ करें और नए उपकरण के लिए स्कैन करें
- `devfsadm -C -v -n` : ड्राई-रन - यह आउटपुट करता है कि क्या बदला जाएगा लेकिन कोई संशोधन नहीं करता

## Links
- More information: https://www.unix.com/man-page/sunos/1m/devfsadm
