use std::io;

/// 入力を読み込む
pub fn read_line() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    Ok(s.trim().to_string())
}
