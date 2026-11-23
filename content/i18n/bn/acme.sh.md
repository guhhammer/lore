# acme.sh

এটি একটি শেল স্ক্রিপ্ট, যা ACME ক্লায়েন্ট প্রোটোকল বাস্তবায়ন করে, এটি `certbot`-এর একটি বিকল্প।

## Usage
- `acme.sh --issue -d example.com -w /webroot/এর/পাথ` : webroot মোড ব্যবহার করে একটি সার্টিফিকেট ইস্যু করুন
- `acme.sh --issue --standalone -d example.com -d www.example.com` : পোর্ট ৮০ ব্যবহার করে standalone মোডে একাধিক ডোমেইনের জন্য একটি সার্টিফিকেট ইস্যু করুন
- `acme.sh --issue --alpn -d example.com` : পোর্ট ৪৪৩ ব্যবহার করে standalone TLS মোডে একটি সার্টিফিকেট ইস্যু করুন
- `acme.sh --issue --nginx -d example.com` : কার্যকর (working) `nginx` কনফিগারেশন ব্যবহার করে একটি সার্টিফিকেট ইস্যু করুন
- `acme.sh --issue --apache -d example.com` : কার্যকর (working) Apache কনফিগারেশন ব্যবহার করে একটি সার্টিফিকেট ইস্যু করুন
- `acme.sh --issue --dns dns_cf -d *.example.com` : স্বয়ংক্রিয় DNS API মোড ব্যবহার করে একটি ওয়াইল্ডকার্ড (`*`) সার্টিফিকেট ইস্যু করুন
- `acme.sh -i -d example.com --key-file /example.com.key/এর/পাথ --fullchain-file /example.com.cer/এর/পাথ --reloadcmd "systemctl force-reload nginx"` : নির্দিষ্ট লোকেশনে সার্টিফিকেট ফাইল ইনস্টল করুন (স্বয়ংক্রিয় সার্টিফিকেট নবায়নের জন্য উপকারী)

## Links
- More information: https://github.com/acmesh-official/acme.sh#2-just-issue-a-cert
