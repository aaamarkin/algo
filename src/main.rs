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

//string findDiff(string str1, string str2)
//{
//// Before proceeding further, make sure str1
//// is not smaller
//if (isSmaller(str1, str2))
//swap(str1, str2);
//
//// Take an empty string for storing result
//string str = "";
//
//// Calculate length of both string
//int n1 = str1.length(), n2 = str2.length();
//
//// Reverse both of strings
//reverse(str1.begin(), str1.end());
//reverse(str2.begin(), str2.end());
//
//int carry = 0;
//
//// Run loop till small string length
//// and subtract digit of str1 to str2
//for (int i=0; i<n2; i++)
//{
//// Do school mathematics, compute difference of
//// current digits
//
//int sub = ((str1[i]-'0')-(str2[i]-'0')-carry);
//
//// If subtraction is less then zero
//// we add then we add 10 into sub and
//// take carry as 1 for calculating next step
//if (sub < 0)
//{
//sub = sub + 10;
//carry = 1;
//}
//else
//carry = 0;
//
//str.push_back(sub + '0');
//}
//
//// subtract remaining digits of larger number
//for (int i=n2; i<n1; i++)
//{
//int sub = ((str1[i]-'0') - carry);
//
//// if the sub value is -ve, then make it positive
//if (sub < 0)
//{
//sub = sub + 10;
//carry = 1;
//}
//else
//carry = 0;
//
//str.push_back(sub + '0');
//}
//
//// reverse resultant string
//reverse(str.begin(), str.end());
//
//return str;
//}

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

//    let h1 = multiply(&a, 2**x.len())?;
//    let h1 = multiply(&a, 2**x.len())?;
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

//    #[test]
//    fn test_mul3() {
//        assert_eq!(multiply("58632", "897256"), Ok("52607913792".to_string()));
//    }
//
//    #[test]
//    fn test_mul4() {
//        assert_eq!(multiply("8", "8"), Ok("64".to_string()));
//    }


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