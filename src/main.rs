use std::io::Write;

fn main() -> std::io::Result<()>{
    loop {
        print!("$ "); std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut tokens = input.split_whitespace();

        match tokens.next() {
            Some("halt") => return Ok(()),
            Some(_) => println!("undefined\n"),
            None => {},
        }

        println!("debug: {tokens:?}\n");
    }
}
