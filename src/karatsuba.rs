use std::num::ParseIntError;
use std::mem;
use std::char;

macro_rules! myprintln {
      () => ();
      ($($arg:tt)*) => (print!(""));
//    () => ($crate::print!("\n"));
//    ($($arg:tt)*) => (println!($($arg)*));
}

macro_rules! myprintln2 {
//      () => ();
//      ($($arg:tt)*) => (print!(""));
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (println!($($arg)*));
}

fn is_smaller(q: &str, w: &str) -> bool {
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

    // Before proceeding further, make sure str1
    // is not smaller

    if is_smaller(&a, &b) {
        mem::swap(  &mut a, &mut b);
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

    let mut sub: u32;
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
            Some(ch) => {myprintln!("ch = {}", ch); str.push(ch); myprintln!("str = {}", str)},
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
            Some(ch) => {myprintln!("ch = {}", ch); str.push(ch); myprintln!("str = {}", str)},
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

    myprintln!("a = {}, b = {}", a, b);
    let mut str:String = "".to_string();

    let n1 = a.len();
    let n2 = b.len();

    let a_rev = a.chars().rev();
    let b_rev = b.chars().rev();

    myprintln!("a_rev = {}, b_rev = {}", a_rev.clone().collect::<String>(), b_rev.clone().collect::<String>());
    let mut carry = 0;

    let a_rev_vec = a_rev.collect::<Vec<char>>();
    let b_rev_vec = b_rev.collect::<Vec<char>>();

    myprintln!("----------------------");

    let mut sum: u32;
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
        myprintln!("a_char_val = {}, b_char_val = {}", a_char_val, b_char_val);
        myprintln!("sum = {}", sum);
        let ch_opt = char::from_digit(sum % 10, 10);

        match ch_opt {
            Some(ch) => {myprintln!("ch = {}", ch); str.push(ch); myprintln!("str = {}", str)},
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
            Some(ch) => {myprintln!("ch = {}", ch); str.push(ch); myprintln!("str = {}", str)},
            _ => panic!()
        }

        carry = sum/10;
    }

    if carry != 0 {
        let carry_opt = char::from_digit(carry, 10);
        match carry_opt {
            Some(ch) => {myprintln!("ch = {}", ch); str.push(ch); myprintln!("str = {}", str)},
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

fn multiply(x: &str, y: &str, debug: bool) -> Result<String, ParseIntError> {

    if debug {
        myprintln2!("___________START___________");
    }

    if x.len() == 1 || y.len() == 1 {
        let a_parsed: i64 = x.parse::<i64>()?;
        let b_parsed: i64 = y.parse::<i64>()?;
        let res = a_parsed * b_parsed;
        let res_str = res.to_string();
        if debug {
            myprintln2!("res_str = {}", res_str);
            myprintln2!("___________FINISH___________");
        }

        return Ok(res_str);
    }

    if debug {
        myprintln2!("x = {}, y = {}", x, y);
    }

    let mut min_len = x.len();

    if x.len() > y.len() {
        min_len = y.len();
    }

    let power = min_len / 2;

    if debug {
        myprintln2!("power = {}", power);
    }



    let x_index = x.len() - power;
    let y_index = y.len() - power;

    let x1 = &x[0..x_index];
    let x0 = &x[x_index..x.len()];

    let y1 = &y[0..y_index];
    let y0 = &y[y_index..y.len()];

    if debug {
        myprintln2!("x1 = {}, x0 = {}", x1, x0);
        myprintln2!("y1 = {}, y0 = {}", y1, y0);
    }


    let a = multiply(x1,y1, false)?;
    let b = multiply(x0,y0, false)?;
    let c = add(x1, x0)?;
    let d = add(y1, y0)?;
    let p = multiply(&c, &d , false)?;

    if debug {
        myprintln2!("a = {}, b = {}, c = {}, d = {}, p = {}", a,b,c,d,p);
    }

    let p_2 = (power * 2) as u32;
    let p_1 = power as u32;

    let mut h1 = a.clone();

    for _x in 0..p_2 {
        h1.push('0');
    }

    let h2 = subtract(&p, &a)?;
    let h3 = subtract(&h2, &b)?;

    let mut h4 = h3.clone();

    for _x in 0..p_1 {
        h4.push('0');
    }


    let h5 = add(&h1, &h4)?;
    let h6 = add(&h5, &b)?;

    if debug {
        myprintln2!("h1 = {}, h2 = {}, h3 = {}, h4 = {}, h5 = {}", h1,h2,h3,h4,h5);
        myprintln2!("res_str = {}", h6);
        myprintln2!("___________FINISH___________");
    }

    return Ok(h6);

}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mul1() {
        assert_eq!(multiply("4", "8", false), Ok("32".to_string()));
    }
    //
    #[test]
    fn test_mul2() {
        assert_eq!(multiply("8", "8", false), Ok("64".to_string()));
    }

    #[test]
    fn test_mul3() {
        assert_eq!(multiply("8", "8", false), Ok("64".to_string()));
    }

    #[test]
    fn test_mul4() {
        assert_eq!(multiply("58632", "897256", true), Ok("52607913792".to_string()));
    }

    #[test]
    fn test_mul5() {
        assert_eq!(multiply("50", "50", false), Ok("2500".to_string()));
    }

    #[test]
    fn test_mul6() {
        assert_eq!(multiply("500", "500", false), Ok("250000".to_string()));
    }


    #[test]
    fn test_mul7() {
        assert_eq!(multiply("852147", "95689562", false), Ok("81541573189614".to_string()));
    }

    #[test]
    fn test_mul8() {
        assert_eq!(multiply("81541573189614", "95689562852147", false), Ok("7802677492790513577878001258".to_string()));
    }

    #[test]
    fn test_mul9() {
        assert_eq!(multiply("3141592653589793238462643383279502884197169399375105820974944592",
                            "2718281828459045235360287471352662497757247093699959574966967627", false),
                   Ok("8539734222673567065463550869546574495034888535765114961879601127067743044893204848617875072216249073013374895871952806582723184".to_string()));
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
        assert_eq!(is_smaller("1", "2"), true);
    }

    #[test]
    fn is_smaller2() {
        assert_eq!(is_smaller("3", "2"), false);
    }

    #[test]
    fn is_smaller3() {
        assert_eq!(is_smaller("121", "2123123"), true);
    }

    #[test]
    fn is_smaller4() {
        assert_eq!(is_smaller("1211231231", "2123123"), false);
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

pub fn run() {
//   let first_num = "3141592653589793238462643383279502884197169399375105820974944592";
//   let second_num = "2718281828459045235360287471352662497757247093699959574966967627";
    let first_num = "50";
    let second_num = "50";
    match multiply(first_num, second_num, false)  {
        Ok(a) =>  println!("Karatsuba = {}", a),
        Err(e) => println!("{}", e.to_string()),
    }
}

