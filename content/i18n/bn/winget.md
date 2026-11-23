# winget

উইন্ডোজ প্যাকেজ ম্যানেজার।

## Usage
- `winget add প্যাকেজ` : একটি প্যাকেজ ইনস্টল করুন (প্যাকেজটি একাধিক সোর্সে এভেলেভেল থাকলে `--source` ব্যবহার করুন)
- `winget rm প্যাকেজ` : একটি প্যাকেজ সরিয়ে ফেলুন (নোট: `uninstall`-এর পরিবর্তে `remove`-ও ব্যবহার করা যেতে পারে)
- `winget show প্যাকেজ` : একটি প্যাকেজ সম্পর্কে তথ্য দেখুন
- `winget search প্যাকেজ` : একটি প্যাকেজ অনুসন্ধান করুন
- `winget upgrade -r` : সব প্যাকেজকে তাদের সর্বশেষ ভার্সনে আপগ্রেড করুন
- `winget ls -s winget` : `winget` দ্বারা পরিচালনাযোগ্য সব ইনস্টল করা প্যাকেজের তালিকা দেখুন
- `winget import|export --import-file|--output ফাইল/এর/পাথ` : একটি ফাইল থেকে প্যাকেজ ইমপোর্ট করুন অথবা ইনস্টল করা প্যাকেজগুলো একটি ফাইলে এক্সপোর্ট করুন
- `winget validate ম্যানিফেস্ট/এর/পাথ` : winget-pkgs রিপোজিটরিতে PR জমা দেওয়ার আগে ম্যানিফেস্ট যাচাই করুন

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
