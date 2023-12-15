
use clap::{command, arg};
use dotenv;

mod lexer;
mod ast;
mod parser;
mod parser2;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let matches = command!()
        .subcommands([
            command!("lexer").arg(arg!(<input>)),
            command!("parser").arg(arg!(<input>)),
            command!("parser2").arg(arg!(<input>)),
            command!("run").arg(arg!(<input>)),
        ]).get_matches();

    match matches.subcommand() {
        Some(("lexer", sub_m)) => if let Some(input) = sub_m.get_one::<String>("input") {
            lexer_single(input);
        } else {
            lexer_repl();
        },
        Some(("parser", sub_m)) => if let Some(input) = sub_m.get_one::<String>("input") {
            parser_single(input);
        } else {
            parser_repl();
        },
         Some(("parser2", sub_m)) => if let Some(input) = sub_m.get_one::<String>("input") {
            parser2_single(input);
        } else {
            parser2_repl();
        },
        Some(("run", sub_m)) => if let Some(input) = sub_m.get_one::<String>("input") {
            println!("Running {}", input);
        } else {
            println!("No input file specified");
        },
        _ => println!("No subcommand was used"),
    }
}

fn lexer_single(input: &str) {
    let lexer = lexer::Lexer::new(input.to_string());
    for tok in lexer {
        println!("{:?}", tok);
    }
}

fn lexer_repl() {
    println!("Keynes LEXER REPL");
    println!("Type in a line of code and press enter to tokenize it.");
    println!("Press Ctrl-C to exit.");
    loop {
        print!(">> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let lexer = lexer::Lexer::new(input);
        for tok in lexer {
            println!("{:?}", tok);
        }
        println!("====================")
    }
}

fn parser_single(input: &str) {
    let mut lexer = lexer::Lexer::new(input.to_string());
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser.parse_program();
    println!("{}", program);
}

fn parser_repl() {
    println!("Keynes PARSER REPL");
    println!("Type in a line of code and press enter to parse it.");
    println!("Press Ctrl-C to exit.");
    loop {
        print!(">> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program();
        println!("{}", program);
        println!("====================")
    }
}

fn parser2_single(input: &str) {
    let mut result = parser2::parse_program(input);
    match result {
        Ok((_,program)) => println!("{:?}", program),
        Err(err) => println!("{:?}", err),
    }
}

fn parser2_repl() {
    println!("Keynes PARSER2 REPL");
    println!("Type in a line of code and press enter to parse it.");
    println!("Press Ctrl-C to exit.");
    loop {
        print!(">> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut result = parser2::parse_program(input);
        match result {
            Ok((_,program)) => println!("{:?}", program),
            Err(err) => println!("{:?}", err),
        }
        println!("====================")
    }
}