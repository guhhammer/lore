# xml canonic

XML ডকুমেন্টকে ক্যানোনিকাল বানিয়ে ফেলুন।

## Usage
- `xml c14n path/to/input.xml|uri >path/to/output.xml` : কমেন্ট সংরক্ষণ করে একটি XML ডকুমেন্টকে ক্যানোনিকাল তৈরি করুন
- `xml c14n --without-comments path/to/input.xml|uri >path/to/output.xml` : কমেন্ট সরিয়ে একটি XML ডকুমেন্টকে ক্যানোনিকাল তৈরি করুন
- `xml c14n --exc-with-comments path/to/input.xml|uri path/to/c14n.xpath` : কমেন্ট সংরক্ষণ করে, একটি ফাইল থেকে XPATH ব্যবহার করে XML এক্সক্লুসিভ ক্যানোনিকাল তৈরি করুন
- `xml c14n --help` : সাহায্য প্রদর্শন

## Links
- More information: https://xmlstar.sourceforge.net/doc/UG/xmlstarlet-ug.html#idm47077139560880
