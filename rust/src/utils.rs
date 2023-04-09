extern crate combine;
use combine::parser::char::{char, digit, spaces};
use combine::{many, ParseError, Parser, Stream, *};

// <expr>   ::= <term> [ ('+'|'-') <term> ]*
// <term>   ::= <factor> [ ('*'|'/') <factor> ]*
// <factor> ::= <number> | '(' <expr> ')'
// <number> :== 1つ以上の数字

fn number<Input>() -> impl Parser<Input, Output = f64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let num = || many1(digit()).map(|x: String| x.parse::<f64>().unwrap());
    (
        num(),
        optional((char('.'), num().map(|x| x * 10_f64.powf(-x.log10().ceil()))))
            .map(|x| x.map(|n| n.1)),
    )
        .map(|(int, frac)| int+frac.unwrap_or(0.0))
}
fn factor<Input>() -> impl Parser<Input, Output = f64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let skip_spaces = || spaces().silent();
    let lex_char = |c| char(c).skip(skip_spaces());
    between(lex_char('('), lex_char(')'), expr()).or(number())
}
fn term<Input>() -> impl Parser<Input, Output = f64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let skip_spaces = || spaces().silent();
    let lex_char = |c| char(c).skip(skip_spaces());
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
fn expr_<Input>() -> impl Parser<Input, Output = f64>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let skip_spaces = || spaces().silent();
    let lex_char = |c| char(c).skip(skip_spaces());
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
        Ok((result, rest)) => {
            if rest == "" {
                Ok(result)
            } else {
                Err(format!("invalid formula: {}", rest))
            }
        }
        Err(s) => Err(s.to_string()),
    }
}
