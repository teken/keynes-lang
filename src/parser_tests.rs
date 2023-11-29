#[cfg(test)]

use super::*;

use test_case::test_case;

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

#[test_case("!5;", "(!5)"; "when number starts with bang")]
#[test_case("-15;", "(-15)"; "when number starts with minus")]
#[test_case("!true;", "(!true)"; "when true starts with bang")]
#[test_case("!false;", "(!false)"; "when false starts with bang")]
fn test_parsing_prefix_expressions(input: &str, expected: &str) {
    let program = lex_and_parse(input);
    let actual = format!("{}", program);
    assert_eq!(actual, expected);
}

#[test_case("5 + 6;", "(5 + 6)"; "when number plus number")]
#[test_case("5 - 6;", "(5 - 6)"; "when number minus number")]
#[test_case("5 * 6;", "(5 * 6)"; "when number multiply number")]
#[test_case("5 / 6;", "(5 / 6)"; "when number divide number")]
#[test_case("5 > 6;", "(5 > 6)"; "when number gt number")]
#[test_case("5 < 6;", "(5 < 6)"; "when number lt number")]
#[test_case("5 == 6;", "(5 == 6)"; "when number eq number")]
#[test_case("5 != 6;", "(5 != 6)"; "when number not eq number")]
#[test_case("true == true", "(true == true)"; "when true eq true")]
#[test_case("true != false", "(true != false)"; "when true not eq false")]
#[test_case("false == false", "(false == false)"; "when false eq false")]
fn test_parsing_infix_expressions(input: &str, expected: &str) {
    let program = lex_and_parse(input);
    let actual = format!("{}", program);
    assert_eq!(actual, expected);
}


#[test_case("-a * b", "((-a) * b)"; "precedence of minus")]
#[test_case("!-a", "(!(-a))"; "precedence of bang")]
#[test_case("a + b + c", "((a + b) + c)"; "precedence of plus")]
#[test_case("a + b - c", "((a + b) - c)"; "precedence of minus and plus")]
#[test_case("a * b * c", "((a * b) * c)"; "precedence of multiply")]
#[test_case("a * b / c", "((a * b) / c)"; "precedence of divide")]
#[test_case("a + b / c", "(a + (b / c))"; "precedence of plus and divide")]
#[test_case("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"; "precedence of plus, multiply, divide and minus")]
#[test_case("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"; "precedence of semicolon")]
#[test_case("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"; "precedence of gt, eq and lt")]
#[test_case("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"; "precedence of lt, not eq and gt")]
#[test_case("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"; "precedence of plus, multiply, eq and plus, multiply")]
#[test_case("true", "true"; "precedence of true")]
#[test_case("false", "false"; "precedence of false")]
#[test_case("3 > 5 == false", "((3 > 5) == false)"; "precedence of gt, eq and false")]
#[test_case("3 < 5 == true", "((3 < 5) == true)"; "precedence of lt, eq and true")]
#[test_case("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"; "precedence of plus and parenthesis")]
#[test_case("(5 + 5) * 2", "((5 + 5) * 2)"; "precedence of plus, parenthesis and multiply")]
#[test_case("2 / (5 + 5)", "(2 / (5 + 5))"; "precedence of divide, parenthesis and plus")]
#[test_case("-(5 + 5)", "(-(5 + 5))"; "precedence of minus and parenthesis")]
#[test_case("!(true == true)", "(!(true == true))"; "precedence of bang and parenthesis")]
#[test_case("a + add(b * c) + d", "((a + add((b * c))) + d)"; "precedence of plus, call and parenthesis")]
#[test_case("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"; "precedence of call and parenthesis 1")]
#[test_case("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))"; "precedence of call and parenthesis 2")]
// #[test_case("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)"; "precedence of index and multiply")]
// #[test_case("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"; "precedence of call, index and multiply")]
fn test_operator_precedence_parsing(input: &str, expected: &str) {
    let program = lex_and_parse(input);
    let actual = format!("{}", program);
    assert_eq!(actual, expected);
}

// #[test_case("if (x < y) { x }", "if (x < y) { x }"; "if statement")]
// #[test_case("if (x < y) { x } else { y }", "if (x < y) { x } else { y }"; "if else statement")]
// #[test_case("if (x < y) { x } else if (x > y) { y }", "if (x < y) { x } else if (x > y) { y }"; "if else if statement")]
// #[test_case("if (x < y) { x } else if (x > y) { y } else { x + y }", "if (x < y) { x } else if (x > y) { y } else { (x + y) }"; "if else if else statement")]
// fn test_if_expression(input: &str, expected: &str) {
//     let program = lex_and_parse(input);
//     let actual = format!("{}", program);
//     assert_eq!(actual, expected);
// }

// #[test_case("fn(x, y) { x + y; }", "fn(x, y) { (x + y); }"; "function literal")]
// #[test_case("fn() { x + y; }", "fn() { (x + y); }"; "function literal without parameters")]
// #[test_case("fn(x, y, z) { x + y + z; }", "fn(x, y, z) { ((x + y) + z); }"; "function literal with multiple parameters")]
// fn test_function_literal(input: &str, expected: &str) {
//     let program = lex_and_parse(input);
//     let actual = format!("{}", program);
//     assert_eq!(actual, expected);
// }

// #[test_case("fn() { return x + y; }", "fn() { return (x + y); }"; "function literal with return statement")]
// #[test_case("fn() { return; }", "fn() { return; }"; "function literal with return statement without expression")]
// fn test_function_literal_with_return_statement(input: &str, expected: &str) {
//     let program = lex_and_parse(input);
//     let actual = format!("{}", program);
//     assert_eq!(actual, expected);
// }

// #[test_case("fn() { x + y; }()", "fn() { (x + y); }()"; "function literal with call expression")]
// #[test_case("fn() { x + y; }(1, 2 * 3, 4 + 5)", "fn() { (x + y); }(1, (2 * 3), (4 + 5))"; "function literal with call expression with arguments")]
// fn test_function_literal_with_call_expression(input: &str, expected: &str) {
//     let program = lex_and_parse(input);
//     let actual = format!("{}", program);
//     assert_eq!(actual, expected);
// }

#[test] 
fn test_call_expression() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let program = lex_and_parse(input);

    assert_eq!(program.statements.len(), 1);

    let mut iter = program.statements.iter();

    let ExpressionStatement { token, expression } = iter.next().unwrap().as_any().downcast_ref::<ExpressionStatement>().unwrap();
    assert_eq!(token, &Token::IDENTIFIER("add".into()));
    
    let CallExpression { token, function, arguments } = expression.as_any().downcast_ref::<CallExpression>().unwrap();
    assert_eq!(token, &Token::IDENTIFIER("add".into()));
    assert_eq!(function.as_any().downcast_ref::<IdentifierLiteral>().unwrap() , &IdentifierLiteral { token: Token::IDENTIFIER("add".into()) });
    assert_eq!(arguments.len(), 3);
    assert_eq!(arguments[0].as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("1".into()), value: 1 });
    
    let InfixExpression { token, left, operator, right } = arguments[1].as_any().downcast_ref::<InfixExpression>().unwrap();
    assert_eq!(token, &Token::MULTIPLY);
    assert_eq!(left.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("2".into()), value: 2 });
    assert_eq!(operator, &InfixOperator::MULTIPLY);
    assert_eq!(right.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("3".into()), value: 3 });
    
    let InfixExpression { token, left, operator, right } = arguments[2].as_any().downcast_ref::<InfixExpression>().unwrap();
    assert_eq!(token, &Token::PLUS);
    assert_eq!(left.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("4".into()), value: 4 });
    assert_eq!(operator, &InfixOperator::PLUS);
    assert_eq!(right.as_any().downcast_ref::<IntegerLiteral>().unwrap() , &IntegerLiteral { token: Token::INTEGER("5".into()), value: 5 });
    
    
}

