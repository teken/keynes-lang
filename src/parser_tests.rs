#[cfg(test)]

use super::*;

#[ctor::ctor]
fn init() {
    dotenv::dotenv().ok();
    env_logger::init();
}

fn lex_and_parse(input: &str) -> Program {
    let mut lexer = Lexer::new(input.into());
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program();
    parser.check_parser_errors();
    program
}

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let mut y = 10;
    let foobar = 838383;
    ";
    let program = lex_and_parse(input);

    assert_eq!(program.statements.len(), 3);
    
    let mut iter = program.statements.iter();

    let LetStatement { token, mutable, name, value } = iter.next().unwrap().as_any().downcast_ref::<LetStatement>().unwrap();
    assert_eq!(token, &Token::LET);
    assert_eq!(mutable, &false);
    assert_eq!(name, &IdentifierLiteral { token: Token::IDENTIFIER("x".into()) });
    assert_eq!(value.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("5".into()), value: 5 });


    let LetStatement { token, mutable, name, value } = iter.next().unwrap().as_any().downcast_ref::<LetStatement>().unwrap();
    assert_eq!(token, &Token::LET);
    assert_eq!(mutable, &true);
    assert_eq!(name, &IdentifierLiteral { token: Token::IDENTIFIER("y".into()) });
    assert_eq!(value.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("10".into()), value: 10 });


    let LetStatement { token, mutable, name, value } = iter.next().unwrap().as_any().downcast_ref::<LetStatement>().unwrap();
    assert_eq!(token, &Token::LET);
    assert_eq!(mutable, &false);
    assert_eq!(name, &IdentifierLiteral { token: Token::IDENTIFIER("foobar".into()) });
    assert_eq!(value.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("838383".into()), value: 838383 });
}

#[test]
fn test_return_statment() {
    let input = "
    return 5;
    return 10;
    return 993322;
    ";
    let program = lex_and_parse(input);
    assert_eq!(program.statements.len(), 3);
    
    let mut iter = program.statements.iter();

    let ReturnStatement { token, expression } = iter.next().unwrap().as_any().downcast_ref::<ReturnStatement>().unwrap();
    assert_eq!(token, &Token::RETURN);
    assert_eq!(expression.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("5".into()), value: 5 });

    let ReturnStatement { token, expression } = iter.next().unwrap().as_any().downcast_ref::<ReturnStatement>().unwrap();
    assert_eq!(token, &Token::RETURN);
    assert_eq!(expression.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("10".into()), value: 10 });

    let ReturnStatement { token, expression } = iter.next().unwrap().as_any().downcast_ref::<ReturnStatement>().unwrap();
    assert_eq!(token, &Token::RETURN);
    assert_eq!(expression.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("993322".into()), value: 993322 });
}

#[test]
fn test_identifier_expression() {
    let input = "foobar;";
    let program = lex_and_parse(input);
    assert_eq!(program.statements.len(), 1);
    
    let mut iter = program.statements.iter();

    let ExpressionStatement { token, expression } = iter.next().unwrap().as_any().downcast_ref::<ExpressionStatement>().unwrap();
    assert_eq!(token, &Token::IDENTIFIER("foobar".into()));
    assert_eq!(expression.as_any().downcast_ref::<IdentifierLiteral>().unwrap() , &IdentifierLiteral { token: Token::IDENTIFIER("foobar".into()) });
}

#[test]
fn test_integer_literal_expression() {
    let input = "5;";
    let program = lex_and_parse(input);
    assert_eq!(program.statements.len(), 1);
    
    let mut iter = program.statements.iter();

    let ExpressionStatement { token, expression } = iter.next().unwrap().as_any().downcast_ref::<ExpressionStatement>().unwrap();
    assert_eq!(token, &Token::INTEGER("5".into()));
    assert_eq!(expression.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("5".into()), value: 5 });
}

#[test]
fn test_parsing_prefix_expressions() {
    let tests = vec![
        ("!5;", "(!5)"),
        ("-15;", "(-15)"),
        ("!true;", "(!true)"),
        ("!false;", "(!false)"),
    ];

    for (input, expected) in tests {
        let program = lex_and_parse(input);
        let actual = format!("{}", program);
        assert_eq!(actual, expected);
        trace!("{} = {} completed", input, expected);
    }
}

#[test]
fn test_parsing_infix_expressions() {
    let tests = vec![
        ("5 + 6;", "(5 + 6)"),
        ("5 - 6;", "(5 - 6)"),
        ("5 * 6;", "(5 * 6)"),
        ("5 / 6;", "(5 / 6)"),
        ("5 > 6;", "(5 > 6)"),
        ("5 < 6;", "(5 < 6)"),
        ("5 == 6;", "(5 == 6)"),
        ("5 != 6;", "(5 != 6)"),
        ("true == true", "(true == true)"),
        ("true != false", "(true != false)"),
        ("false == false", "(false == false)"),
    ];

    for (input, expected) in tests {
        let program = lex_and_parse(input);
        let actual = format!("{}", program);
        assert_eq!(actual, expected);
        trace!("{} = {} completed", input, expected);
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        ("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        ("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        ("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))"),
        ("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)"),
        ("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
    ];

    for (input, expected) in tests {
        let program = lex_and_parse(input);
        let actual = format!("{}", program);
        assert_eq!(actual, expected);
        trace!("{} = {} completed", input, expected);
    }
}

