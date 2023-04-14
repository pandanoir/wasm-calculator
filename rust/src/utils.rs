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

macro_rules! chars {
    ( $x:expr ) => { { char($x).skip(spaces().silent()) } };
    ( $x:expr, $( $xs:expr),* ) => {
        { one_of(vec![$x, $($xs)*]).skip(spaces().silent()) }
    };
}

// <expr>   ::= <term> [ ('+'|'-') <term> ]*
// <term>   ::= <factor> [ ('*'|'/') <factor> ]*
// <factor> ::= <number> | '(' <expr> ')'
// <number> :== 1つ以上の数字
fn number<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    let sign = chars!('-').map(|_| -1.0);
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
    between(chars!('('), chars!(')'), expr()).or(number())
}
fn term<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    (factor(), many((chars!('*', '/'), factor()))).map(|x: (f64, Vec<(char, f64)>)| {
        x.1.iter().fold(
            x.0,
            |acc, &(op, x)| if op == '*' { acc * x } else { acc / x },
        )
    })
}
fn expr_<Input: Stream<Token = char>>() -> impl ExprParser<Input, f64> {
    (term(), many((chars!('+', '-'), term()))).map(|x: (f64, Vec<(char, f64)>)| {
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
