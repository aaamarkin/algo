use std::io::BufReader;
use std::io::BufRead;


pub fn read_file(file_name: &str) -> Result<Vec<i32>, std::io::Error> {

    println!("In file {}", file_name);
    let mut vector: Vec<i32> = Vec::new();;

    let f = (std::fs::File::open(file_name))?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let s = l.parse().unwrap();
        vector.push(s);
    }
    return Ok(vector);
}