use std::io::BufReader;
use std::io::BufRead;
use crate::mincut;
use crate::mincut::ALE;

pub fn read_file_to_vec(file_name: &str) -> Result<Vec<i32>, std::io::Error> {

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

pub fn read_file_to_ale_list(file_name: &str) -> Result<Vec<ALE>, std::io::Error> {

    println!("In file {}", file_name);
    let mut vector: Vec<ALE> = Vec::new();

    let f = (std::fs::File::open(file_name))?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let split = l.split_whitespace().collect::<Vec<&str>>();;
        let mut nodes:Vec<i32> = Vec::new();
        nodes.push(split[0].parse().unwrap());
        let mut edges:Vec<i32> = Vec::new();
//        println!("line {:?}", l);
//        println!("split {:?}", split);
        for index in 1..split.len() {
            let number = split[index].parse().unwrap();
            edges.push(number);
        }

        let ale = ALE {
            nodes,
            edges
        };

        vector.push(ale);
    }
    return Ok(vector);
}

//pub fn read_file_to_array(file_name: &str) -> Result<&mut [i32], std::io::Error> {
//
//    println!("In file {}", file_name);
//    let mut vector: Vec<i32> = Vec::new();;
//
//    let f = (std::fs::File::open(file_name))?;
//    let file = BufReader::new(&f);
//    for line in file.lines() {
//        let l = line.unwrap();
//        let s = l.parse().unwrap();
//        vector.push(s);
//    }
//    return Ok(vector.as_mut());
//}