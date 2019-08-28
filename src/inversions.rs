use std::io::BufReader;
use std::io::BufRead;

fn read_file(file_name: &str) -> Result<Vec<i32>, std::io::Error> {

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

fn inversions(vector: Vec<i32>) -> (Vec<i32>, i64){
    if vector.len() == 1 {
        return (vector, 0);
    }

    let pivot = vector.len() / 2;

    let v1 = vector[0..pivot].to_vec();
    let v2 = vector[pivot..].to_vec();

    let (vec1, vec1_inv) = inversions(v1);
    let (vec2, vec2_inv) = inversions(v2);

    return merge_injections(vec1, vec2, vec1_inv + vec2_inv);
}

#[cfg(test)]
fn merge_sort(vector: Vec<i32>) -> Vec<i32>{
    if vector.len() == 1 {
        return vector;
    }

    let pivot = vector.len() / 2;

    let v1 = vector[0..pivot].to_vec();
    let v2 = vector[pivot..].to_vec();

    let vec1 = merge_sort(v1);
    let vec2 = merge_sort(v2);

    return merge(vec1, vec2);
}

#[cfg(test)]
fn merge(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    return merge_injections(v1, v2, 0).0;
}

#[cfg(test)]
fn injections(v1: Vec<i32>, v2: Vec<i32>) -> i64 {
    return merge_injections(v1, v2, 0).1;
}

fn merge_injections(v1: Vec<i32>, v2: Vec<i32>, counted_inv: i64) -> (Vec<i32>, i64) {

    let mut result: Vec<i32> = Vec::new();
    let mut counter = 0;
    let mut injections_counter = counted_inv;
    for (index, value) in v1.iter().enumerate() {

        if counter >= v2.len(){
            result.push(*value);
            ;
        } else if *value < v2[counter]{
            result.push(*value);
        } else if counter < v2.len() {

            let dif = v2.len() - counter;

            let injection_len = v1.len() - index ;

            for _ in 0..dif {
                if *value >= v2[counter] {
                    result.push(v2[counter]);
                    counter+=1;
                    injections_counter+=injection_len as i64;
                } else {
                    break;
                }
            }

            result.push(*value);
        }
    }


    if counter < v2.len(){
        let dif = v2.len() - counter;

        for _ in 0..dif {
            result.push(v2[counter]);
            counter+=1;
        }

    }

    return (result, injections_counter as i64);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_merge0() {
        assert_eq!(merge(vec![1], vec![5]), vec![1,5]);
    }

    #[test]
    fn test_merge1() {
        assert_eq!(merge(vec![1,2,3], vec![5,6,7]), vec![1,2,3,5,6,7]);
    }

    #[test]
    fn test_merge2() {
        assert_eq!(merge(vec![1,2,7], vec![5,6,8]), vec![1,2,5,6,7,8]);
    }

    #[test]
    fn test_merge3() {
        assert_eq!(merge(vec![1,5,7], vec![2,3,8]), vec![1,2,3,5,7,8]);
    }

    #[test]
    fn test_merge4() {
        assert_eq!(merge(vec![5,6,7], vec![1,2,3]), vec![1,2,3,5,6,7]);
    }

    #[test]
    fn test_merge5() {
        assert_eq!(merge(vec![1,2,3,4], vec![5,6,7]), vec![1,2,3,4,5,6,7]);
    }

    #[test]
    fn test_merge6() {
        assert_eq!(merge(vec![1,2,3,4,8,9], vec![5,6,7]), vec![1,2,3,4,5,6,7,8,9]);
    }

    #[test]
    fn test_merge7() {
        assert_eq!(merge(vec![1,2,3,4,8,9], vec![5,6,7,10,11,12,13]), vec![1,2,3,4,5,6,7,8,9,10,11,12,13]);
    }
    #[test]
    fn test_merge8() {
        assert_eq!(merge(vec![1,2,3,4,8,9,14,15,16], vec![5,6,7,10,11,12,13,20,30,40,50,60]), vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,20,30,40,50,60]);
    }





    #[test]
    fn test_injections0() {
        assert_eq!(injections(vec![1], vec![5]), 0);
    }

    #[test]
    fn test_injections1() {
        assert_eq!(injections(vec![1, 2, 3], vec![5, 6, 7]), 0);
    }

    #[test]
    fn test_injections2() {
        assert_eq!(injections(vec![1, 2, 7], vec![5, 6, 8]), 2);
    }

    #[test]
    fn test_injections3() {
        assert_eq!(injections(vec![1, 6, 7], vec![4, 5, 8]), 4);
    }

    #[test]
    fn test_injections4() { assert_eq!(injections(vec![4, 5, 6], vec![1, 2, 3]), 9); }

    #[test]
    fn test_injections5() { assert_eq!(injections(vec![1, 5, 6], vec![2, 7, 8]), 2); }



    #[test]
    fn test_merge_sort0() {
        assert_eq!(merge_sort(vec![1,2,3]), vec![1,2,3]);
    }

    #[test]
    fn test_merge_sort1() {
        assert_eq!(merge_sort(vec![3,2,1]), vec![1,2,3]);
    }

    #[test]
    fn test_merge_sort2() {
        assert_eq!(merge_sort(vec![2,3,1]), vec![1,2,3]);
    }

    #[test]
    fn test_merge_sort3() {
        assert_eq!(merge_sort(vec![2,3,0,1]), vec![0,1,2,3]);
    }

    #[test]
    fn test_merge_sort4() {
        assert_eq!(merge_sort(vec![2,3,0,1,8,7,5,6,3,4]), vec![0,1,2,3,3,4,5,6,7,8]);
    }

    #[test]
    fn test_merge_sort5() {
        assert_eq!(merge_sort(vec![5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16]), vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,20,30,40,50,60]);
    }

    #[test]
    fn test_merge_sort6() {
        assert_eq!(merge_sort(vec![5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16,5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16]),
                   vec![1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13,14,14,15,15,16,16,20,20,30,30,40,40,50,50,60,60]);
    }



    #[test]
    fn test_inversions0() {
        assert_eq!(inversions(vec![1,2,3]).1, 0);
    }

    #[test]
    fn test_inversions1() {
        assert_eq!(inversions(vec![2,1,3]).1, 1);
    }

    #[test]
    fn test_inversions2() {
        assert_eq!(inversions(vec![3,1,2]).1, 2);
    }

    #[test]
    fn test_inversions3() {
        assert_eq!(inversions(vec![1, 5, 6, 4, 20]).1, 2);
    }

    #[test]
    fn test_inversions4() {
        assert_eq!(inversions(vec![2, 4, 1, 3, 5 ]).1, 3);
    }

    #[test]
    fn test_inversions5() {
        assert_eq!(inversions(vec![2, 4, 1, 3, 5, 0 ]).1, 8);
    }


}

pub fn run() {

    let file_name = "IntegerArray.txt";

    let vector = read_file(file_name).unwrap();

    println!("Inversions - {}", inversions(vector).1)
}