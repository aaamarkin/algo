use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;
use std::fs;
use std::path::Path;

fn readFile(fileName: &str) -> Result<Vec<i32>, std::io::Error> {

    println!("In file {}", fileName);
    let mut vector: Vec<i32> = Vec::new();;

    let f = (std::fs::File::open(fileName))?;
    let mut file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let s = l.parse().unwrap();
        vector.push(s);
    }
    return Ok(vector);
//    let contents = fs::read_to_string("IntegerArraySmall.txt")
//        .expect("Something went wrong reading the file");
//
//    println!("With text:\n{}", contents);
}
fn main() {

    let fileName = "IntegerArray.txt";

    let mut vector = readFile(fileName).unwrap();

    println!("Length - {}", vector.len())

}