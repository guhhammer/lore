# a2query

ডেবিয়ান (Debian) ভিত্তিক অপারেটিং সিস্টেমে অ্যাপাচি (Apache)-এর রানটাইম কনফিগারেশন জানার জন্য ব্যবহৃত কমান্ড।

## Usage
- `sudo a2query -m` : সক্রিয় (enabled) অ্যাপাচি মডিউলগুলোর তালিকা দেখতে
- `sudo a2query -m মডিউলের_নাম` : নির্দিষ্ট কোনো মডিউল ইনস্টল আছে কিনা পরীক্ষা করতে
- `sudo a2query -s` : সক্রিয় ভার্চুয়াল হোস্টগুলোর তালিকা দেখতে
- `sudo a2query -M` : বর্তমানে সক্রিয় মাল্টি প্রসেসিং মডিউল (Multi Processing Module) দেখতে
- `sudo a2query -v` : অ্যাপাচির সংস্করণ (version) দেখতে

## Links
- More information: https://manned.org/a2query
