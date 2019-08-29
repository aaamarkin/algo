use crate::file::read_file;
//use crate::{myprintln,myprintln2};

fn sort(vector: &mut [i32]) {

    println!("Input = {:?}", vector);

    if vector.len() <= 1 {
        println!("Final output = {:?}", vector);
        return;
    } else if vector.len() == 2 {
        if vector[0] <= vector[1]{
            println!("Final output = {:?}", vector);
            return;
        } else {
            vector.swap(0,1);
            println!("Final output = {:?}", vector);
            return;
        }
    }

    let pivot_idx = get_pivot_index(&vector);

    vector.swap(0,pivot_idx);

    let pivotVal = vector[0];

    let mut lt_index = 1;
    let mut gt_index = 1;

    for index in 1..vector.len() {

        let value = vector[index];

        if value <= pivotVal {

            if gt_index != lt_index {

                vector.swap(lt_index,index);

            }

            lt_index += 1;
            gt_index += 1;

        } else {

            gt_index += 1;

        }
    }


    vector.swap(lt_index - 1,0);

    println!("Output = {:?}", vector);

    sort(&mut vector[0..lt_index - 1]);
    sort(&mut vector[lt_index..]);

    println!("Final output = {:?}", vector);
}

fn get_pivot_index(vector: & [i32]) -> usize {

    return 0;
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_sort0() {
        let t = &mut [1,2,3];
        sort(t);
        assert_eq!(t, &mut [1,2,3]);
    }

    #[test]
    fn test_sort1() {
        let t = &mut [3,2,1];
        sort(t);
        assert_eq!(t, &mut [1,2,3]);
    }

    #[test]
    fn test_sort2() {
        let t = &mut [2,3,1];
        sort(t);
        assert_eq!(t , &mut [1,2,3]);
    }

    #[test]
    fn test_sort3() {
        let t = &mut [2,3,0,1];
        sort(t);
        assert_eq!(t, &mut [0,1,2,3]);
    }

    #[test]
    fn test_sort4() {
        let t = &mut [2,3,0,1,8,7,5,6,3,4];
        sort(t);
        assert_eq!(t, &mut [0,1,2,3,3,4,5,6,7,8]);
    }

    #[test]
    fn test_sort5() {
        let t = &mut [5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16];
        sort(t);

        assert_eq!(t, &mut [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,20,30,40,50,60]);
    }
//
//    #[test]
//    fn test_sort6() {
//        let t = &mut [5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16,5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16];
//        sort(t);
//
//        assert_eq!(t, &mut [1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13,14,14,15,15,16,16,20,20,30,30,40,40,50,50,60,60]);
//    }


}

pub fn run() {

    let t = &mut [5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16,5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16];
    sort(t);
//    println!("Quicksort ");

}