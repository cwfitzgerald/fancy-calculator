use nom::IResult;
use nom::multi::{many0, fold_many0};
use nom::branch::alt;
use nom::sequence::tuple;
use nom::bytes::complete::{take_while1, is_a};
use nom::character::complete::{multispace0, one_of};
use nom::combinator::{all_consuming, opt};

pub fn math(input: &str) -> IResult<&str, f64> {
    let (input, result) = all_consuming(add_sub)(input)?;
    Ok((input, (result * 100_000.).round() / 100_000.))
}

fn add_sub(input: &str) -> IResult<&str, f64> {
    let (input, lhs) = mult_div(input)?;
    let folder = |acc, (delim, val)| {
        match delim {
            '+' => acc + val,
            '-' => acc - val,
            '−' => acc - val,
            _ => unreachable!(),
        }
    };
    let (input, sum) = fold_many0(tuple((one_of("+-−"), mult_div)), lhs, folder)(input)?;

    Ok((input, sum))
}

fn mult_div(input: &str) -> IResult<&str, f64> {
    let (input, lhs) = number(input)?;
    let folder = |acc, (delim, val)| {
        match delim {
            '*' => acc * val,
            'x' => acc * val,
            '×' => acc * val,
            '/' => acc / val,
            '÷' => acc / val,
            _ => unreachable!(),
        }
    };
    let (input, sum) = fold_many0(tuple((one_of("*x×/÷"), number)), lhs, folder)(input)?;

    Ok((input, sum))
}

fn number(input: &str) -> IResult<&str, f64> {
    let (input, _) = multispace0(input)?;
    let (input, negation) = opt(one_of("+-−"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = take_while1(|c| "0123456789.".contains(c))(input)?;
    let (input, _) = multispace0(input)?;

    let float: f64 = value.parse().unwrap();

    let float: f64 = match negation {
        Some('-') | Some('−') => -float,
        _ => float,
    };

    Ok((input, float))
}

#[cfg(test)]
mod test {
    use crate::math::math;

    #[test]
    fn simple_maths() {
        assert_eq!(math("1 + 2 + 3 + 4"), Ok(("", 10.0)));
        assert_eq!(math("1 + 2 - 3 * 4 / 5"), Ok(("", 0.6)));
        assert_eq!(math("1 / 2 * 3 - 4 + 5"), Ok(("", 2.5)));
        assert_eq!(math("8 - 3"), Ok(("", 5.0)));
        assert_eq!(math("-3"), Ok(("", -3.0)));
        assert_eq!(math("-3 * -2"), Ok(("", 6.0)));
    }
}