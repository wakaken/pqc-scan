//! Self-contained bilingual HTML shell shared by reports and their rule guide.
use crate::ReportLanguage;

pub(crate) fn page(en: &str, ja: &str, language: ReportLanguage) -> String {
    let (lang, alternate_lang, active, alternate) = if language == ReportLanguage::Ja {
        ("ja", "en", ja, en)
    } else {
        ("en", "ja", en, ja)
    };
    let title = if language == ReportLanguage::Ja {
        "pqc-scan レポート"
    } else {
        "pqc-scan report"
    };
    format!(
        r#"<!doctype html><html lang="{lang}" data-theme="light"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{title}</title><style>{base}
{style}</style></head><body><div class="report-settings" data-settings hidden><label>Language / 言語<select data-language aria-label="Language / 言語"><option value="en">English</option><option value="ja">日本語</option></select></label><label>Theme / 表示<select data-theme-select aria-label="Theme / 表示"><option value="light">Light / ライト</option><option value="dark">Dark / ダーク</option></select></label></div>{active}<template data-report-translation lang="{alternate_lang}">{alternate}</template><script>{controller}
{preferences}</script></body></html>"#,
        base = crate::html_report::css(),
        style = include_str!("compact.css"),
        controller = include_str!("compact.js"),
        preferences = include_str!("presentation.js")
    )
}

// Encode a relative filename, never an absolute output directory, in local links.
pub(crate) fn filename_url(path: &std::path::Path) -> String {
    use std::fmt::Write;
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let mut out = String::new();
    for byte in name.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(byte as char);
        } else {
            write!(out, "%{byte:02X}").expect("writing to a String");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_links_encode_filename_and_do_not_include_the_output_directory() {
        assert_eq!(
            super::filename_url(std::path::Path::new("/private/output/a #日.html")),
            "a%20%23%E6%97%A5.html"
        );
        assert_eq!(
            super::filename_url(std::path::Path::new("report.rules.html")),
            "report.rules.html"
        );
    }
}
