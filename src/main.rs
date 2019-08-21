use std::num::ParseIntError;
use std::mem;
use std::str::Chars;
use std::char;
use std::ops::Deref;

fn isSmaller(q: &str, w: &str) -> bool {
    // Calculate lengths of both string
    let n1 = q.len();
    let n2 = w.len();

    if n1 < n2 {
        return true;
    }

    if n2 < n1{
        return false;
    }

    let q_vec = q.chars().collect::<Vec<char>>();
    let w_vec = w.chars().collect::<Vec<char>>();

    for x in q_vec.iter().enumerate(){

        let q_char_val = *x.1;
        let w_char_val = w_vec[x.0];
        // Do school mathematics, compute sum of
        // current digits and carry
        let q_int_opt = q_char_val.to_digit(10);
        let w_int_opt = w_char_val.to_digit(10);

        match q_int_opt {
            Some(q_int) => match w_int_opt {
                Some(w_int) => {
                    if q_int < w_int { return true }
                    else if q_int > w_int { return false }
                },
                _ => panic!(),
            }
            _ => panic!(),
        }

    }
    return false;
}

fn subtract<'l>(q: &'l str, w: &'l str) -> Result<String, ParseIntError> {

    let mut a = q.clone().to_string();
    let mut b = w.clone().to_string();

    let mut was_swapped = false;

    // Before proceeding further, make sure str1
    // is not smaller

    if isSmaller(&a, &b) {
        mem::swap(  &mut a, &mut b);
        was_swapped = true;
    }

    // Take an empty string for storing result
    let mut str:String = "".to_string();

    // Calculate length of both string
    let n1 = a.len();
    let n2 = b.len();

    let a_rev = a.chars().rev();
    let b_rev = b.chars().rev();

    let mut carry = 0;

    let a_rev_vec = a_rev.collect::<Vec<char>>();
    let b_rev_vec = b_rev.collect::<Vec<char>>();

    let mut sub: u32 = 0;
    // Run loop till small string length
    // and subtract digit of str1 to str2
    for x in b_rev_vec.iter().enumerate() {
        // Do school mathematics, compute difference of
        // current digits

        let b_char_val = *x.1;
        let a_char_val = a_rev_vec[x.0];
        // Do school mathematics, compute sum of
        // current digits and carry
        let a_int_opt = a_char_val.to_digit(10);
        let b_int_opt = b_char_val.to_digit(10);

        match a_int_opt {
            Some(a_int) => match b_int_opt {
                Some(b_int) => {
                    // If subtraction is less then zero
                    // we add then we add 10 into sub and
                    // take carry as 1 for calculating next step
                    if a_int < b_int || a_int < carry || a_int < (b_int + carry)
                    {
                        sub = (a_int + 10) - b_int - carry;
                        carry = 1;
                    }
                    else {
                        sub = a_int - b_int - carry;
                        carry = 0;
                    }


                },
                _ => panic!(),
            }
            _ => panic!(),
        }



        let ch_opt = char::from_digit(sub, 10);

        match ch_opt {
            Some(ch) => {println!("ch = {}", ch); str.push(ch); println!("str = {}", str)},
            _ => panic!()
        }
    }

    // subtract remaining digits of larger number
    for x in n2..n1 {
        let a_char_val = a_rev_vec[x];
        let a_int_opt = a_char_val.to_digit(10);
        match a_int_opt {
            Some(a_int) => {
                if a_int < carry
                {
                    sub = (a_int + 10) - carry ;
                    carry = 1;
                }
                else
                {
                    sub = a_int - carry ;
                    carry = 0;
                }
            },
            _ => panic!(),
        }
        // if the sub value is -ve, then make it positive



        let ch_opt = char::from_digit(sub, 10);
        match ch_opt {
            Some(ch) => {println!("ch = {}", ch); str.push(ch); println!("str = {}", str)},
            _ => panic!()
        }
    }

    // reverse resultant string
    return Ok(str.chars().rev().collect::<String>());
}

fn add<'l>(q: &'l str, w: &'l str) -> Result<String, ParseIntError> {

    let mut a = q.clone().to_string();
    let mut b = w.clone().to_string();

    let mut was_swapped = false;

    if a.len() > b.len() {
        mem::swap(  &mut a, &mut b);
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
        mem::swap( &mut a, &mut b);
    }

    return Ok(str.chars().rev().collect::<String>());
}
//3141592
//27182818
//3141 * 1000 + 592

fn multiply(x: &str, y: &str) -> Result<String, ParseIntError> {

    if x.len() == 1 || y.len() == 1 {
        let a_parsed: i64 = x.parse::<i64>()?;
        let b_parsed: i64 = y.parse::<i64>()?;
        let res = a_parsed * b_parsed;
        let res_str = res.to_string();
        return Ok(res_str);
    }


    let power = x.len() / 2;
    let f_index = x.len() - power;

    let mut x1 = &x[0..f_index];
    let mut x0 = &x[f_index..x.len()];

    let mut y1 = &y[0..f_index];
    let mut y0 = &y[f_index..y.len()];

    let a = multiply(x1,y1)?;
    let b = multiply(x0,y0)?;
    let c = add(x1, x0)?;
    let d = add(y1, y0)?;
    let p = multiply(&c, &d )?;

    let char1 = char::from_digit(2u32.pow(x.len()), 10)?;
    let char2 = char::from_digit(2u32.pow(power), 10)?;
    let h1 = multiply(&a, &char1.to_string())?;
    let h2 = subtract(&p, &a)?;
    let h3 = subtract(&h2, &b)?;
    let h4 = multiply(&h3, &char2.to_string())?;
    let h5 = add(&h1, &h4)?;
    let h6 = add(&h5, &b)?;

    return Ok(h6);

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
    fn test_sum0() {
        assert_eq!(add("3", "4"), Ok("7".to_string()));
    }
    #[test]
    fn test_sum1() {
        assert_eq!(add( "321", "432"), Ok("753".to_string()));
    }

    #[test]
    fn test_sum2() {
        assert_eq!(add("741", "468"), Ok("1209".to_string()));
    }

    #[test]
    fn test_sum3() {
        assert_eq!(add("852147", "9568"), Ok("861715".to_string()));
    }

    #[test]
    fn test_sum4() {
        assert_eq!(add( "852147", "95689562"), Ok("96541709".to_string()));
    }

    #[test]
    fn is_smaller1() {
        assert_eq!(isSmaller( "1", "2"), true);
    }

    #[test]
    fn is_smaller2() {
        assert_eq!(isSmaller( "3", "2"), false);
    }

    #[test]
    fn is_smaller3() {
        assert_eq!(isSmaller( "121", "2123123"), true);
    }

    #[test]
    fn is_smaller4() {
        assert_eq!(isSmaller( "1211231231", "2123123"), false);
    }


    #[test]
    fn test_subtract1() {
        assert_eq!(subtract("741", "468"), Ok("273".to_string()));
    }

    #[test]
    fn test_subtract2() {
        assert_eq!(subtract( "321", "432"), Ok("111".to_string()));
    }

    #[test]
    fn test_subtract3() {
        assert_eq!(subtract("852147", "9568"), Ok("842579".to_string()));
    }

    #[test]
    fn test_subtract4() {
        assert_eq!(subtract( "852147", "95689562"), Ok("94837415".to_string()));
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