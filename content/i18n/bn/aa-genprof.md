# aa-genprof

প্রোগ্রামের আচরণ পর্যবেক্ষণ করে অ্যাপআর্মর (AppArmor) নিরাপত্তা প্রোফাইল তৈরি করুন।

## Usage
- `sudo aa-genprof প্রোগ্রামের_পাথ` : কোনো প্রোগ্রামের জন্য প্রোফাইল জেনারেট করুন
- `sudo aa-genprof -d /প্রোফাইলসমূহ/এর/পাথ প্রোগ্রামের_পাথ` : নির্দিষ্ট একটি ডিরেক্টরিতে প্রোফাইল সংরক্ষণ করুন
- `sudo aa-genprof -f /লগফাইল/এর/পাথ প্রোগ্রামের_পাথ` : প্রোফাইলিংয়ের জন্য কাস্টম লগফাইল নির্ধারণ করুন
- `aa-genprof -h` : সাহায্য প্রদর্শন

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-genprof.8
