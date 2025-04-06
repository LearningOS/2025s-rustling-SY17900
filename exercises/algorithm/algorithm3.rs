/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn down<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) {
    let mut parent = start;
    let mut child = parent*2 + 1;
    while child <= end {
        if child + 1 <= end && arr[child] <= arr[child+1] {
            child += 1;
        }
        if arr[parent] >= arr[child] {
            return;
        } else {
            arr.swap(parent, child);
            parent = child;
            child = parent*2 + 1;
        }
    }
}

fn sort<T: PartialOrd + Clone>(array: &mut [T]) {
	//TODO
    let end_index = array.len() - 1;
    for i in (0..array.len() / 2).rev() {
        down(array, i, end_index);
    }

    for i in (1..end_index + 1).rev() {
        array.swap(0, i);
        down(array, 0, i - 1);
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