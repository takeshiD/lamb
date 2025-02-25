use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::map,
    error::{context, VerboseError},
    multi::many0,
    sequence::delimited,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum BuiltinOp {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    Num(i32),
    Boolean(bool),
    Operater(BuiltinOp),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    SelfEvaluation(Atom),
    Application(Box<Expr>, Vec<Expr>),
}

fn parse_number(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
    let (input, val) = digit1(input)?;
    Ok((input, Atom::Num(val.parse::<i32>().unwrap())))
}

fn parse_bool(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
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

fn parse_operater(input: &str) -> IResult<&str, Atom, VerboseError<&str>> {
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

fn parse_selfeval(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map(
        alt((parse_number, parse_bool, parse_operater)),
        Expr::SelfEvaluation,
    )(input)
}

fn parse_procedure(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (input, _) = char('(')(input)?;
    let (input, car) = context("string", delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, cdr) = many0(delimited(multispace0, parse_expr, multispace0))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, Expr::Application(Box::new(car), cdr)))
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    alt((parse_procedure, parse_selfeval))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::integer_positive("123", Ok(("", Atom::Num(123))))]
    #[case::integer_negative("-123", Ok(("", Atom::Num(-123))))]
    fn test_parse_number(#[case] input: &str, #[case] expected: IResult<&str, Atom, Error<&str>>) {
        assert_eq!(parse_number(input), expected);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("#t"), Ok(("", Atom::Boolean(true))));
        assert_eq!(parse_bool("#f"), Ok(("", Atom::Boolean(false))));
    }

    #[test]
    fn test_parse_operater() {
        assert_eq!(
            parse_operater("+"),
            Ok(("", Atom::Operater(BuiltinOp::Plus)))
        );
        assert_eq!(
            parse_operater("-"),
            Ok(("", Atom::Operater(BuiltinOp::Minus)))
        );
        assert_eq!(
            parse_operater("*"),
            Ok(("", Atom::Operater(BuiltinOp::Times)))
        );
        assert_eq!(
            parse_operater("/"),
            Ok(("", Atom::Operater(BuiltinOp::Divide)))
        );
    }

    #[test]
    fn test_parse_selfeval() {
        assert_eq!(
            parse_selfeval("123"),
            Ok(("", Expr::SelfEvaluation(Atom::Num(123))))
        );
        assert_eq!(
            parse_selfeval("#t"),
            Ok(("", Expr::SelfEvaluation(Atom::Boolean(true))))
        );
        assert_eq!(
            parse_selfeval("+"),
            Ok(("", Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))))
        );
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(
            parse_procedure("(+ 1 2)"),
            Ok((
                "",
                Expr::Application(
                    Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))),
                    vec![
                        Expr::SelfEvaluation(Atom::Num(1)),
                        Expr::SelfEvaluation(Atom::Num(2))
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_expr("(+(* 1 2)(- 3 4))"),
            Ok((
                "",
                Expr::Application(
                    Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Plus))),
                    vec![
                        Expr::Application(
                            Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Times))),
                            vec![
                                Expr::SelfEvaluation(Atom::Num(1)),
                                Expr::SelfEvaluation(Atom::Num(2))
                            ]
                        ),
                        Expr::Application(
                            Box::new(Expr::SelfEvaluation(Atom::Operater(BuiltinOp::Minus))),
                            vec![
                                Expr::SelfEvaluation(Atom::Num(3)),
                                Expr::SelfEvaluation(Atom::Num(4))
                            ]
                        )
                    ]
                )
            ))
        );
    }
}
