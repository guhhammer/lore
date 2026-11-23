# aa-mergeprof

অ্যাপআর্মর (AppArmor) নিরাপত্তা প্রোফাইল ফাইলগুলোকে প্রোফাইল ডিরেক্টরিতে একত্র (merge) করুন।

## Usage
- `sudo aa-mergeprof ফাইল১ ফাইল২ ...` : এক বা একাধিক প্রোফাইল ফাইল ডিফল্ট প্রোফাইল ডিরেক্টরিতে মার্জ করুন
- `sudo aa-mergeprof -d /প্রোফাইলসমূহ/এর/পাথ ফাইল১ ফাইল২ ...` : নির্দিষ্ট একটি ডিরেক্টরিতে প্রোফাইল ফাইল মার্জ করুন
- `aa-mergeprof -h` : সাহায্য প্রদর্শন

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-mergeprof.8
