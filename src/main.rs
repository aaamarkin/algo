use std::num::ParseIntError;
use std::mem;

fn add<'l>(a: &'l mut &'l str, b: &'l mut &'l str) -> Result<String, ParseIntError> {

    let mut was_swapped = false;
    if a.len() > b.len() {
        mem::swap( a, b);
        was_swapped = true;
    }


    let mut str:String = "".to_string();

    let n1 = a.len();
    let n2 = b.len();

    let a_rev = a.chars().rev().collect::<String>();
    let b_rev = b.chars().rev().collect::<String>();

    let mut carry = 0;

    for x in 0..n1 {
        // Do school mathematics, compute sum of
        // current digits and carry
        let sum = (a_rev.as_bytes()[x])+(b_rev.as_bytes()[x])+carry;
        str.push(char::from((sum % 10).to_ascii_lowercase()));

        // Calculate carry for next step
        carry = sum/10;
    }


    for x in n1..n2 {

        let sum = ((b_rev.as_bytes()[x])+carry);
        str.push(char::from((sum % 10).to_ascii_lowercase()));
        carry = sum/10;
    }

    if carry != 0 {
        str.push(char::from((carry).to_ascii_lowercase()));
    }


//    let a_parsed: i64 = a.parse::<i64>()?;
//    let b_parsed: i64 = b.parse::<i64>()?;
//
//    let res = a_parsed * b_parsed;
//    let res_str = res.to_string();

    if was_swapped {
        mem::swap( a, b);
    }

    return Ok(str.chars().rev().collect::<String>());
}
//3141592
//27182818
//3141 * 1000 + 592

fn multiply(a: &str, b: &str) -> Result<String, ParseIntError> {
    let power = a.len() / 2;
    let f_index = a.len() - power;
    let a1 = &a[0..f_index];
    let a0 = &a[f_index..a.len()];


    let a_parsed: i64 = a.parse::<i64>()?;
    let b_parsed: i64 = b.parse::<i64>()?;

    let res = a_parsed * b_parsed;
    let res_str = res.to_string();

    return Ok(res_str);
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mul1() {
        assert_eq!(multiply("4", "8"), Ok("32".to_string()));
    }

    #[test]
    fn test_mul2() {
        assert_eq!(multiply("8", "8"), Ok("64".to_string()));
    }

    #[test]
    fn test_mul3() {
        assert_eq!(multiply("58632", "897256"), Ok("52607913792".to_string()));
    }

    #[test]
    fn test_mul4() {
        assert_eq!(multiply("8", "8"), Ok("64".to_string()));
    }



    #[test]
    fn test_sum1() {
        assert_eq!(add(&mut "4", &mut "8"), Ok("12".to_string()));
    }
}

fn main() {
    let first_num = "3141592653589793238462643383279502884197169399375105820974944592";
    let second_num = "2718281828459045235360287471352662497757247093699959574966967627";
    let first_num = "3";
    let second_num = "2";
    match multiply(first_num, second_num)  {
        Ok(a) =>  println!("{}", a),
        Err(e) => println!("{}", e.to_string()),
    }

}