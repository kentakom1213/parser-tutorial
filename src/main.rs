use parser_tutorial::{parse, read_line, render_error, tokenize};

fn main() {
    let Ok(input) = read_line().map_err(|e| e.to_string()) else {
        println!("io error");
        return;
    };

    let tokens = match tokenize(&input) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{}", render_error(&input, &e));
            return;
        }
    };
    let expr = match parse(tokens) {
        Ok(expr) => expr,
        Err(e) => {
            println!("{}", render_error(&input, &e));
            return;
        }
    };

    println!("{expr:#?}");
}
