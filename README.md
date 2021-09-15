# sada(صدأ)

![نبغا احد يسويلنا رسمة]()

Aren't you _متعب_ from writing Rust programs in English? Would you like to try something different, in an exotic and funny way? Would you want to bring some Arabic
touch to your programs?

**sada(صدأ)** (Arabic for _Rust_) is here to save your day, as it allows you to
write Rust programs in Arabic, using Arabic keywords, Arabic function names, Arabic idioms.

You don't feel at ease using only Arabic words? Don't worry! Arabic Rust is fully compatible with English-Rust, so you can mix both at your convenience.

Here's an example of what can be achieved with Arabic:

### trait and impl (aka سمه و تنفيذ )

```rust
sada::sada! {
    خارِجي صندوق sada;

    استخدم std::collections::قاموس ك قاموسي;

    سمه مفتاح_القيمة {
        دالة اكتب(&ذات, مفتاح: نص, قيمة: نص);
        دالة اقرأ(&ذات, مفتاح: نص) -> نتيجة<خيار<&نص>, نص>;
    }

    لا_حركة متغير قاموس: خيار<قاموسي<نص, نص>> = لا_شيء;

    بنية محددة;

    تنفيذ مفتاح_القيمة ل محددة {
        دالة اكتب(&ذات, مفتاح: نص, قيمة: نص) {
            دع قاموسي = غير_امن {
                قاموس.خذ_او_ادخل_ب(افتراضي::طبيعي)
            };
            قاموسي.ادخل(مفتاح, قيمة);
        }

        دالة اقرأ(&ذات, مفتاح: نص) -> نتيجة<خيار<&نص>, نص> {
            اذا دع بعض(قاموسي) = غير_امن { قاموس.ك_مرجِع() } {
                حسنا(قاموسي.جلب(&مفتاح))
            } آخر {
                خطأ("لا يمكن جلب القاموس".إلى())
            }
        }
    }
}
```


### :امثلة اخرى

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

.وبس والله


## ليش؟

ليش لا؟

## :لغات اخرى

- French(الأصل): [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)

## :شروط الترخيص

[WTFPL](http://www.wtfpl.net/)

