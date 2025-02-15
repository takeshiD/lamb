use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1, one_of},
    combinator::map,
    error::Error,
    multi::many0,
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
enum BuiltinOp {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
enum Atom {
    Num(i32),
    Boolean(bool),
    Operater(BuiltinOp),
}

#[derive(Debug)]
enum Expr {
    SelfEvaluation(Atom),
    Application(Box<Expr>, Vec<Expr>),
}

fn parse_number(input: &str) -> IResult<&str, Atom, Error<&str>> {
    let (input, val) = digit1(input)?;
    Ok((input, Atom::Num(val.parse::<i32>().unwrap())))
}

fn parse_bool(input: &str) -> IResult<&str, Atom, Error<&str>> {
    let (input, val) = alt((tag("#t"), tag("#f")))(input)?;
    Ok((
        input,
        match val {
            "#t" => Atom::Boolean(true),
            "#f" => Atom::Boolean(false),
            _ => unreachable!("Invalid input: not boolean"),
        },
    ))
}

fn parse_operater(input: &str) -> IResult<&str, Atom, Error<&str>> {
    let (input, val) = one_of("+-*/")(input)?;
    Ok((
        input,
        match val {
            '+' => Atom::Operater(BuiltinOp::Plus),
            '-' => Atom::Operater(BuiltinOp::Minus),
            '*' => Atom::Operater(BuiltinOp::Times),
            '/' => Atom::Operater(BuiltinOp::Divide),
            _ => unreachable!(),
        },
    ))
}

fn parse_selfeval(input: &str) -> IResult<&str, Expr, Error<&str>> {
    map(
        alt((parse_number, parse_bool, parse_operater)),
        Expr::SelfEvaluation,
    )(input)
}

fn parse_pair(input: &str) -> IResult<&str, Expr, Error<&str>> {
    let (input, _) = char('(')(input)?;
    let (input, car) = delimited(multispace0, parse_expr, multispace0)(input)?;
    let (input, cdr) = many0(delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Application(Box::new(car), cdr)))
}

fn parse_expr(input: &str) -> IResult<&str, Expr, Error<&str>> {
    alt((parse_pair, parse_selfeval))(input)
}

fn main() {
    let input = "(+ (* 1 2) (- 3 4))";
    match parse_expr(input) {
        Ok((_, expr)) => {
            println!("Input: {input}\nExpr: {expr:#?}");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
