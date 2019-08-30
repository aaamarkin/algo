use crate::file::read_file_to_vec;
use crate::{myprintln, myprintln2};

fn sort(array: &mut [i32], mut num_of_comp: i32) -> i32 {

    myprintln!("Input = {:?}", array);

    if array.len() <= 1 {
        myprintln!("Final output = {:?}", array);
        return num_of_comp;
    } else if array.len() == 2 {
        if array[0] <= array[1]{
            myprintln!("Final output = {:?}", array);
            return num_of_comp;
        } else {
            array.swap(0,1);
            myprintln!("Final output = {:?}", array);
            return num_of_comp;
        }
    }

    let pivot_idx = get_pivot_index(&array);

    array.swap(0,pivot_idx);

    let pivot_val = array[0];

    let mut lt_index = 1;
    let mut gt_index = 1;

    for index in 1..array.len() {

        let value = array[index];

        if value <= pivot_val {

            if gt_index != lt_index {

                array.swap(lt_index,index);

            }

            lt_index += 1;
            gt_index += 1;

        } else {

            gt_index += 1;

        }
    }


    array.swap(lt_index - 1,0);

    myprintln!("Output = {:?}", array);

    num_of_comp += array[0..lt_index - 1].len() as i32 - 1;
    num_of_comp += array[lt_index..].len() as i32 - 1;

    sort(&mut array[0..lt_index - 1], num_of_comp);
    sort(&mut array[lt_index..], num_of_comp);

    myprintln!("Final output = {:?}", array);

    return num_of_comp;
}

fn get_pivot_index(vector: & [i32]) -> usize {

    let mut tmo_vec: Vec<(i32, usize)> = Vec::new();

    let v_left = (vector[0], 0);
    let v_right = (vector[vector.len() - 1], vector.len() - 1);
    let v_middle = (vector[vector.len() / 2], vector.len() / 2);

    tmo_vec.push(v_left);
    tmo_vec.push(v_right);
    tmo_vec.push(v_middle);

    tmo_vec.sort_by(|a, b| a.0.cmp(&b.0));

    return tmo_vec[1].1 as usize;
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_sort0() {
        let t = &mut [1,2,3];
        sort(t, 0);
        assert_eq!(t, &mut [1,2,3]);
    }

    #[test]
    fn test_sort1() {
        let t = &mut [3,2,1];
        sort(t, 0);
        assert_eq!(t, &mut [1,2,3]);
    }

    #[test]
    fn test_sort2() {
        let t = &mut [2,3,1];
        sort(t, 0);
        assert_eq!(t , &mut [1,2,3]);
    }

    #[test]
    fn test_sort3() {
        let t = &mut [2,3,0,1];
        sort(t, 0);
        assert_eq!(t, &mut [0,1,2,3]);
    }

    #[test]
    fn test_sort4() {
        let t = &mut [2,3,0,1,8,7,5,6,3,4];
        sort(t, 0);
        assert_eq!(t, &mut [0,1,2,3,3,4,5,6,7,8]);
    }

    #[test]
    fn test_sort5() {
        let t = &mut [5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16];
        sort(t, 0);

        assert_eq!(t, &mut [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,20,30,40,50,60]);
    }

    #[test]
    fn test_pivot0() {
        let idx = get_pivot_index(&mut [1,2,3]);

        assert_eq!(idx, 1);
    }

    #[test]
    fn test_pivot1() {
        let idx = get_pivot_index(&mut [2,1,3]);

        assert_eq!(idx, 0);
    }

    #[test]
    fn test_pivot2() {
        let idx = get_pivot_index(&mut [3,1,2]);

        assert_eq!(idx, 2);
    }
}

pub fn run() {

    let file_name = "QuickSort.txt";

    let mut vec = read_file_to_vec(file_name).unwrap();
    let array = vec.as_mut();

    let t = &mut [5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16,5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16];
    let num = sort(array,0);
    myprintln2!("Quicksort {:?}", num);

}