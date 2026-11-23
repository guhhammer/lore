# am

एंड्रॉइड गतिविधि प्रबंधक।

## Usage
- `am start -n com.android.settings/.Settings` : एक विशिष्ट गतिविधि प्रारंभ करें
- `am start -a android.intent.action.VIEW -d tel:123` : एक गतिविधि शुरू करें और उसमें डेटाd पास करें
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : किसी विशिष्ट क्रिया और श्रेणीc से मेल खाती गतिविधि प्रारंभ करें
- `am to-uri -a android.intent.action.VIEW -d tel:123` : एक उद्देश्य को यूआरआई में बदलें

## Links
- More information: https://developer.android.com/tools/adb#am
