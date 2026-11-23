# g++

ترجمة ملفات مصدر C++. جزء من حزمة GCC (مجموعة مترجمات جنو).

## Usage
- `g++ path/to/source1.cpp path/to/source2.cpp ... -o path/to/output_executable` : ترجمة ملف أو عدة ملفات مصدر إلى ملف تنفيذي
- `g++ path/to/source.cpp -Wall -o path/to/output_executable` : إظهار جميع التحذيرات
- `g++ path/to/source.cpp -Wall -g -Og -o path/to/output_executable` : إظهار التحذيرات الشائعة، تضمين رموز التصحيح، وتحسين لا يؤثر في إزالة التصحيح
- `g++ path/to/source.cpp -std=c++98|c++11|c++14|c++17 -o path/to/output_executable` : اختيار معيار اللغة المستهدف (C++98/C++11/C++14/C++17)
- `g++ path/to/source.cpp -o path/to/output_executable -Ipath/to/header -Lpath/to/library -llibrary_name` : تضمين مسارات للرؤوس والمكتبات والربط بمكتبة
- `g++ -c path/to/source1.cpp path/to/source2.cpp ... && g++ -o path/to/output_executable path/to/source1.o path/to/source2.o ...` : ترجمة ثم ربط عدة ملفات مصدر على خطوتين
- `g++ path/to/source.cpp -O1|2|3|fast -o path/to/output_executable` : تحسين الأداء عند الترجمة
- `g++ --version` : عرض الإصدار

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/C_002b_002b-Dialect-Options.html
