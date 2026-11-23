# choco install

Chocolatey ব্যবহার করে এক বা একাধিক প্যাকেজ ইনস্টল করুন।

## Usage
- `choco install প্যাকেজ১ প্যাকেজ२ ...` : এক বা একাধিক প্যাকেজ ইনস্টল করুন
- `choco install path\to\packages_file.config` : একটি কাস্টম কনফিগারেশন ফাইল থেকে প্যাকেজ ইনস্টল করুন
- `choco install path\to\file` : একটি নির্দিষ্ট `.nuspec` বা `.nupkg` ফাইল ইনস্টল করুন
- `choco install প্যাকেজ --version version` : একটি প্যাকেজের নির্দিষ্ট ভার্সন ইনস্টল করুন
- `choco install package --allow-multiple` : একটি প্যাকেজের একাধিক ভার্সন ইনস্টল করার অনুমতি দিন
- `choco install প্যাকেজ -y` : সব প্রম্পট স্বয়ংক্রিয়ভাবে নিশ্চিত করুন
- `choco install প্যাকেজ -s source_url|alias` : প্যাকেজ গ্রহণের জন্য একটি কাস্টম সোর্স নির্দিষ্ট করুন
- `choco install প্যাকেজ -u ইউজারনেম -p পাসওয়ার্ড` : অথেনটিকেশনের জন্য একটি ব্যবহারকারীর নাম এবং পাসওয়ার্ড দিন

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
