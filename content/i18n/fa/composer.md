# composer

ابزاری بسته محور برای مدیریت وابستگی های پروژه های PHP.

## Usage
- `composer init` : ساخت یک فایل `composer.json` به صورت کنشگرا
- `composer require user/package` : اضافه کردن یک بسته به عنوان وابستگی به این پروژه، همچنین یک ورودی به `composer.json` وارد می کند
- `composer install` : نصب تمام وابستگی های این پروژه که در `composer.json` هستند و `composer.lock` را ایجاد می کند
- `composer remove user/package` : حذف یک بسته از این پروژه، وابستگی مربوط به آنرا از `composer.json` و `composer.lock` حذف می کند
- `composer update` : بروزرسانی تمام وابستگی های این پروژه که در `composer.json` هستند و یادداشت کردن نسخه های جدید در فایل `composer.lock`
- `composer update --lock` : فقط `composer.lock` را بروزرسانی می کند بعد از این که `composer.json` را به صورت دستی بروزرسانی کردید
- `composer why-not user/package` : اطلاعات بیشتری درباره دلیل نصب نشدن یک وابستگی ارائه می دهد
- `composer self-update` : بروزرسانی کامپوزر به آخرین نسخه اش

## Links
- More information: https://getcomposer.org/doc/03-cli.md
