use std::num::ParseIntError;



fn multiply(a: &str, b: &str) -> Result<String, ParseIntError> {
    let a_parsed: i32 = a.parse::<i32>()?;
    let b_parsed: i32 = b.parse::<i32>()?;

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
}

fn main() {
    let first_num = "3141592653589793238462643383279502884197169399375105820974944592";
    let second_num = "2718281828459045235360287471352662497757247093699959574966967627";
//    let first_num = "3";
//    let second_num = "2";
    match multiply(first_num, second_num)  {
        Ok(a) =>  println!("{}", a),
        Err(e) => println!("{}", e.to_string()),
    }

}