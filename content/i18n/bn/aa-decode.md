# aa-decode

অ্যাপআর্মর (AppArmor) অডিট লগগুলোকে মানুষের পড়ার উপযোগী (human-readable) ফরম্যাটে রূপান্তর করার জন্য ব্যবহৃত কমান্ড।

## Usage
- `aa-decode হেক্স_স্ট্রিং` : কোনো হেক্স (hex) স্ট্রিং ডিকোড করতে
- `sudo aa-decode লগফাইল` : কোনো লগ ফাইল ডিকোড করতে
- `sudo aa-decode - <লগফাইল` : `stdin` (যেমন রিডাইরেক্ট করা ফাইল) থেকে লগ ডিকোড করতে
- `aa-decode -h` : সাহায্য প্রদর্শন

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-decode.8
