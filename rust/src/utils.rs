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

// <expr>   ::= <term> [ ('+'|'-') <term> ]*
// <term>   ::= <factor> [ ('*'|'/') <factor> ]*
// <factor> ::= <number> | '(' <expr> ')'
// <number> :== 1つ以上の数字
fn lex_char<Input: Stream<Token = char>>(c: char) -> impl ExprParser<Input, char> {
    let skip_spaces = || spaces().silent();
    char(c).skip(skip_spaces())
}

fn number<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    let sign = optional(lex_char('-')).map(|x| if let Some(_) = x { -1.0 } else { 1.0 });
    let integer = char('0')
        .map(|_| 0.0_f64)
        .or((one_of("123456789".chars()), many(digit()))
            .map(|x: (char, String)| (x.0.to_string() + &x.1).parse::<f64>().unwrap()));
    let decimal =
        many1(digit()).map(|x: String| x.parse::<f64>().unwrap() * 10_f64.powf(-(x.len() as f64)));
    (
        sign,
        integer,
        optional((char('.'), decimal)).map(|x| x.map(|n| n.1).unwrap_or(0.0)),
    )
        .map(|(sign, int, frac)| sign * (int + frac))
}
fn factor<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    between(lex_char('('), lex_char(')'), expr()).or(number())
}
fn term<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    (
        factor(),
        many::<Vec<(char, f64)>, _, _>((lex_char('*').or(lex_char('/')), factor())),
    )
        .map(|x| {
            x.1.iter().fold(
                x.0,
                |acc, &(op, x)| if op == '*' { acc * x } else { acc / x },
            )
        })
}
fn expr_<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    (
        term(),
        many::<Vec<(char, f64)>, _, _>((lex_char('+').or(lex_char('-')), term())),
    )
        .map(|x| {
            x.1.iter().fold(
                x.0,
                |acc, &(op, x)| if op == '+' { acc + x } else { acc - x },
            )
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
