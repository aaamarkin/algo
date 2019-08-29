use crate::file::read_file;

fn sort(vector: &mut Vec<i32>) -> &Vec<i32>{

    if vector.len() == 1 {
        return vector;
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


    vector.swap(lt_index,0);

    sort(&mut vector[0..lt_index].to_vec());
    sort(&mut vector[lt_index + 1..].to_vec());

    return vector;
}

fn get_pivot_index(vector: &Vec<i32>) -> usize {

    return 0;
}

#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_sort0() {
        assert_eq!(sort(&mut vec![1,2,3]), &mut vec![1,2,3]);
    }

    #[test]
    fn test_sort1() {
        assert_eq!(sort(&mut vec![3,2,1]), &mut vec![1,2,3]);
    }

//    #[test]
//    fn test_sort2() {
//        assert_eq!(sort(vec![2,3,1]), vec![1,2,3]);
//    }
//
//    #[test]
//    fn test_sort3() {
//        assert_eq!(sort(vec![2,3,0,1]), vec![0,1,2,3]);
//    }
//
//    #[test]
//    fn test_sort4() {
//        assert_eq!(sort(vec![2,3,0,1,8,7,5,6,3,4]), vec![0,1,2,3,3,4,5,6,7,8]);
//    }
//
//    #[test]
//    fn test_sort5() {
//        assert_eq!(sort(vec![5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16]), vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,20,30,40,50,60]);
//    }
//
//    #[test]
//    fn test_sort6() {
//        assert_eq!(sort(vec![5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16,5,6,7,10,11,12,13,20,30,40,50,60,1,2,3,4,8,9,14,15,16]),
//                   vec![1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,10,10,11,11,12,12,13,13,14,14,15,15,16,16,20,20,30,30,40,40,50,50,60,60]);
//    }


}

pub fn run() {

    println!("Quicksort ");

}