use std::num::ParseIntError;
use std::mem;
use std::str::Chars;
use std::char;

fn add<'l>(a: &'l mut &'l str, b: &'l mut &'l str) -> Result<String, ParseIntError> {

    let mut was_swapped = false;
    if a.len() > b.len() {
        mem::swap( a, b);
        was_swapped = true;
    }

    println!("a = {}, b = {}", a, b);
    let mut str:String = "".to_string();

    let n1 = a.len();
    let n2 = b.len();

    let a_rev = a.chars().rev();
    let b_rev = b.chars().rev();

    println!("a_rev = {}, b_rev = {}", a_rev.clone().collect::<String>(), b_rev.clone().collect::<String>());
    let mut carry = 0;

    let a_rev_vec = a_rev.collect::<Vec<char>>();
    let b_rev_vec = b_rev.collect::<Vec<char>>();

    println!("----------------------");

    let mut sum: u32 = 0;
    for x in a_rev_vec.iter().enumerate(){

        let a_char_val = *x.1;
        let b_char_val = b_rev_vec[x.0];
        // Do school mathematics, compute sum of
        // current digits and carry
        let a_int_opt = a_char_val.to_digit(10);
        let b_int_opt = b_char_val.to_digit(10);

        match a_int_opt {
            Some(a_int) => match b_int_opt {
                Some(b_int) => sum = a_int + b_int + carry,
                _ => panic!(),
            }
            _ => panic!(),
        }
        println!("a_char_val = {}, b_char_val = {}", a_char_val, b_char_val);
        println!("sum = {}", sum);
        let ch_opt = char::from_digit(sum % 10, 10);

        match ch_opt {
            Some(ch) => {println!("ch = {}", ch); str.push(ch); println!("str = {}", str)},
            _ => panic!()
        }

        // Calculate carry for next step
        carry = sum/10;
    }


    for x in n1..n2 {

        let b_char_val = b_rev_vec[x];
        let b_int_opt = b_char_val.to_digit(10);

        match b_int_opt {
            Some(b_int) => sum = b_int + carry,
            _ => panic!(),
        }
        let ch_opt = char::from_digit(sum % 10, 10);
        match ch_opt {
            Some(ch) => {println!("ch = {}", ch); str.push(ch); println!("str = {}", str)},
            _ => panic!()
        }

        carry = sum/10;
    }

    if carry != 0 {
        let carry_opt = char::from_digit(carry, 10);
        match carry_opt {
            Some(ch) => {println!("ch = {}", ch); str.push(ch); println!("str = {}", str)},
            _ => panic!()
        }
    }

    if was_swapped {
        mem::swap( a, b);
    }

    return Ok(str.chars().rev().collect::<String>());
}
//3141592
//27182818
//3141 * 1000 + 592

fn multiply(x: &str, y: &str) -> Result<String, ParseIntError> {
    let power = x.len() / 2;
    let f_index = x.len() - power;
    let x1 = &x[0..f_index];
    let x0 = &x[f_index..x.len()];

    let y1 = &y[0..f_index];
    let y0 = &y[f_index..y.len()];

    let a = x1 * y1;
    let b = x0 * y0;
    let p = (x1 + x0) * (y1 + y0);

//    return a * 2**x.len() + (p - a - b)*(2**power) + b;

    let a_parsed: i64 = x.parse::<i64>()?;
    let b_parsed: i64 = y.parse::<i64>()?;

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
        assert_eq!(add(&mut "321", &mut "432"), Ok("753".to_string()));
    }

    #[test]
    fn test_sum2() {
        assert_eq!(add(&mut "741", &mut "468"), Ok("1209".to_string()));
    }

    #[test]
    fn test_sum3() {
        assert_eq!(add(&mut "852147", &mut "9568"), Ok("861715".to_string()));
    }

    #[test]
    fn test_sum4() {
        assert_eq!(add(&mut "852147", &mut "95689562"), Ok("96541709".to_string()));
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