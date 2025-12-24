use crate::TokenSpan;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub(crate) span: TokenSpan,
    pub(crate) message: String,
}

impl ParseError {
    pub fn new(span: TokenSpan, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

pub fn render_error(input: &str, err: &ParseError) -> String {
    let mut out = String::new();

    // 末尾の改行を削除
    let line = input.trim().trim_end_matches('\n');

    // 該当行
    out += line;
    out += "\n";

    // 2行目：^ マーカー
    let mut marker = String::new();

    // start まで空白を詰める
    for (i, b) in line.as_bytes().iter().enumerate() {
        if i >= err.span.start {
            break;
        }
        // タブは幅を保つ
        marker.push(if *b == b'\t' { '\t' } else { ' ' });
    }

    // span 長（最低 1）
    let len = (err.span.end.max(err.span.start + 1)) - err.span.start;
    marker.push_str(&"^".repeat(len));

    out.push_str(&marker);
    out.push('\n');

    // 3行目：エラーメッセージ
    out.push_str("error: ");
    out.push_str(&err.message);

    out
}
