extern crate combine;
use combine::parser::char::{char, digit, spaces};
use combine::{many, ParseError, Parser, Stream, *};

trait ExprParser<Input: Stream<Token = char>, Output>: Parser<Input, Output = Output>
where
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
}
impl<Input: Stream<Token = char>, Output, T: Parser<Input, Output = Output>>
    ExprParser<Input, Output> for T
{
}

fn lex_char<Input: Stream<Token = char>>(c: char) -> impl ExprParser<Input, char> {
    char(c).skip(spaces().silent())
}
macro_rules! chars {
    ( $x:expr ) => {{
        one_of($x.chars()).skip(spaces().silent())
    }};
}

// <expr>   ::= <term> [ ('+'|'-') <term> ]*
// <term>   ::= <factor> [ ('*'|'/') <factor> ]*
// <factor> ::= <number> | '(' <expr> ')'
// <number> :== 1つ以上の数字
fn number<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    let sign = lex_char('-').map(|_| -1.0);
    let integer = choice!(
        char('0').map(|_| 0.0_f64),
        (one_of("123456789".chars()), many(digit()))
            .map(|x: (char, String)| { format!("{}{}", x.0, x.1).parse::<f64>().unwrap() })
    );
    let decimal =
        many1(digit()).map(|x: String| x.parse::<f64>().unwrap() * 10_f64.powf(-(x.len() as f64)));

    (
        optional(sign).map(|x| x.unwrap_or(1.0)),
        integer,
        optional((char('.'), decimal)).map(|x| x.map(|n| n.1).unwrap_or(0.0)),
    )
        .map(|(sign, int, frac)| sign * (int + frac))
}
fn factor<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    between(lex_char('('), lex_char(')'), expr())
        .or(number())
        .skip(spaces().silent())
}
fn term<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    (factor(), many((chars!("*/"), factor())))
        .map(|x: (f64, Vec<(char, f64)>)| {
            x.1.iter().fold(
                x.0,
                |acc, &(op, x)| if op == '*' { acc * x } else { acc / x },
            )
        })
        .skip(spaces().silent())
}
fn expr_<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    spaces().then(|_| {
        (term(), many((chars!("+-"), term())))
            .map(|x: (f64, Vec<(char, f64)>)| {
                x.1.iter().fold(
                    x.0,
                    |acc, &(op, x)| if op == '+' { acc + x } else { acc - x },
                )
            })
            .skip(spaces().silent())
    })
}
parser! {
    fn expr[Input]()(Input) -> f64 where [Input: Stream<Token = char>] { expr_() }
}

pub fn parse_expr(expr_str: &str) -> Result<f64, String> {
    match expr().parse(expr_str) {
        Ok((result, rest)) if rest == "" => Ok(result),
        Ok((_, rest)) => Err(format!("invalid formula: {}", rest)),
        Err(s) => Err(s.to_string()),
    }
}

#[test]
fn parse_number() {
    assert_eq!(parse_expr("0.1"), Ok(0.1));
    assert_eq!(parse_expr("0.0001"), Ok(0.0001));
    assert_eq!(parse_expr("10.0001"), Ok(10.0001));
    assert_eq!(parse_expr("3.2"), Ok(3.2));
    assert_eq!(parse_expr("-3.2"), Ok(-3.2));
    assert_eq!(parse_expr("32"), Ok(32.0));
    assert_eq!(parse_expr("(64)"), Ok(64.0));
}
#[test]
fn expression() {
    assert_eq!(parse_expr("1.2+3.4*5.6"), Ok(20.24));
    assert_eq!(parse_expr("1.2*(-2)"), Ok(-2.4));
    assert_eq!(parse_expr("2*3"), Ok(6.0));
    assert_eq!(parse_expr("3*(2*2)"), Ok(12.0));
    assert_eq!(parse_expr("-3*(2*2)"), Ok(-12.0));
    assert_eq!(parse_expr("123+456"), Ok(579.0));
    assert_eq!(parse_expr("1+2*(3+4)"), Ok(15.0));
    assert_eq!(parse_expr("12/4"), Ok(3.0));
    assert_eq!(parse_expr("12-4"), Ok(8.0));
}
#[test]
fn skip_whitespaces() {
    assert_eq!(parse_expr("   1.2 +  3.4  * 5.6   "), Ok(20.24));
    assert_eq!(parse_expr("1.2 * (  -  2   )   "), Ok(-2.4));
    assert_eq!(parse_expr("2  *   3   "), Ok(6.0));
    assert_eq!(parse_expr("   -   3  *  (2*2)    "), Ok(-12.0));
    assert_eq!(parse_expr("    12  /  4  "), Ok(3.0));
    assert_eq!(parse_expr("    12  -  4  "), Ok(8.0));
}
#[test]
fn invalid_values() {
    assert_eq!(
        parse_expr("00.1"),
        Err(String::from("invalid formula: 0.1"))
    );
    assert_eq!(
        parse_expr("0.1.2.3"),
        Err(String::from("invalid formula: .2.3"))
    );
    assert_eq!(parse_expr("0 .1"), Err(String::from("invalid formula: .1")));
}
