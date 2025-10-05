/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:Ord>(array: &mut [T]){
	//TODO
    if array.len() <= 1 {
        return;
    }

    let length = array.len();
    for i in (0..length/2-1).rev() {
        shift_down(array, i, length);
    }

    for i in (0..length).rev(){
        array.swap(0, i);
        shift_down(array,0,i);
    }
}

fn shift_down<T: Ord>(array: &mut [T], start: usize, end: usize){
    let mut index = start;
    while index * 2 + 1 < end {
        let mut child = 2*index+1;

        if child+1 < end && array[child+1] > array[child]{
            child += 1;
        }

        if array[child] > array[index]{
            array.swap(child, index);
            index = child;
        } else {
            return;
        }

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}