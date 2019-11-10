use nom::IResult;
use nom::multi::{many0, fold_many0};
use nom::branch::alt;
use nom::sequence::tuple;
use nom::bytes::complete::{take_while1, is_a};
use nom::character::complete::{multispace0, one_of};
use nom::combinator::all_consuming;

fn math(input: &str) -> IResult<&str, f64> {
    let (input, result) = add_sub(input)?;
    Ok((input, (result * 100_000.).round() / 100_000.))
}

fn add_sub(input: &str) -> IResult<&str, f64> {
    let (input, lhs) = mult_div(input)?;
    let folder = |acc, (delim, val)| {
        match delim {
            '+' => acc + val,
            '-' => acc - val,
            _ => unreachable!(),
        }
    };
    let (input, sum) = fold_many0(tuple((one_of("+-"), mult_div)), lhs, folder)(input)?;

    Ok((input, sum))
}

fn mult_div(input: &str) -> IResult<&str, f64> {
    let (input, lhs) = number(input)?;
    let folder = |acc, (delim, val)| {
        dbg!(acc, delim, val);
        match delim {
            '*' => acc * val,
            'x' => acc * val,
            '/' => acc / val,
            _ => unreachable!(),
        }
    };
    let (input, sum) = fold_many0(tuple((one_of("*x/"), number)), lhs, folder)(input)?;
    dbg!(sum);


    Ok((input, sum))
}

fn number(input: &str) -> IResult<&str, f64> {
    let (input, _) = multispace0(input)?;
    let (input, value) = take_while1(|c| "0123456789.".contains(c))(input)?;
    let (input, _) = multispace0(input)?;

    let float: f64 = value.parse().unwrap();

    Ok((input, float))
}

#[cfg(test)]
mod test {
    use crate::math::math;

    #[test]
    fn simple_maths() {
//        assert_eq!(math("1 + 2 + 3 + 4"), Ok(("", 10.0)));
//        assert_eq!(math("1 + 2 - 3 * 4 / 5"), Ok(("", 0.6)));
        assert_eq!(math("1 / 2 * 3 - 4 + 5"), Ok(("", 2.5)));
    }
}