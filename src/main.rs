use parser_tutorial::{parse, read_line};

fn main() -> Result<(), String> {
    let input = read_line().map_err(|e| e.to_string())?;

    let tokens = parse(&input)?;

    println!("{tokens:?}");

    Ok(())
}
