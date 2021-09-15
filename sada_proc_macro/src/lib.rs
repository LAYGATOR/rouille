use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "خطأ" => "Err",
        "حسنا" => "Ok",
        "نص" => "String",
        "قاموس" => "HashMap",
        "افتراضي" => "Default",
        "خطئ" => "Error",
        "خيار" => "Option",
        "بعض" => "Some",
        "لا_شيء" => "None",
        "نتيجة" => "Result",
        "ذاتي" => "Self",
        "اطبع" => "println",
        "كسر" => "break",
        "غير_متزامنة" => "async",
        "انتظر" => "await",
        "حلقه" => "loop",
        "حرك" => "move",
        "صندوق" => "crate",
        "كود_بعيد_المنال" => "unreachable_code",
        "ك" => "as",
        "ثابت" => "const",
        "سمه" => "trait",
        "غير_امن" => "unsafe",
        "في" => "in",
        "من" => "from",
        "ديناميكي" => "dyn",
        "فك" => "unwrap",
        "طبيعي" => "default",
        "ك_مرجِع" => "as_ref",
        "دخ" => "io",
        "خارِجي" => "extern",
        "خاطئ" => "false",
        "دالة" => "fn",
        "أعلى" => "super",
        "ادخل" => "insert",
        "جلب" => "get",
        "اسمح" => "allow",
        "هلع" => "panic",
        "وحدة" => "mod",
        "متغير" => "mut",
        "جديد" => "new",
        "اين" => "where",
        "ل" => "for",
        "خذ_او_ادخل_ب" => "get_or_insert_with",
        "رئيسي" => "main",
        "عام" => "pub",
        "ماذا" => None?,
        "إرجاع" => "return",
        "تنفيذ" => "impl",
        "مرجع" => "ref",
        "طابق" => "match",
        "اذا" => "if",
        "آخر" => "else",
        "ذات" => "self",
        "دع" => "let",
        "لا_حركة" => "static",
        "بنية" => "struct",
        "توقع" => "expect",
        "بينما" => "while",
        "استخدم" => "use",
        "إلى" => "into",
        "صحيح" => "true",
        "تعداد" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn sada(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
