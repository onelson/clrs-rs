//! Insertion Sort (Chapter 2: Getting Started)
//!
//! ```text
//! for j = 2 to A.length
//!     key = A[j]
//!     // insert A[j] into the sorted sequence A[1 .. j - 1]
//!     i = j - 1
//!     while i > 0 and A[i] > key
//!         a[i + 1] = A[i]
//!         i = i - 1
//!         A[i + 1] = key
//! ```

/// Plain port of the pseudocode.
pub fn sort(data: &mut Vec<i32>) {
    for j in 1..data.len() {
        let key = data[j];
        let mut i: i32 = j as i32 - 1;
        while i >= 0 && data[i as usize] > key {
            data[(i + 1) as usize] = data[i as usize];
            i = i - 1;
            data[(i + 1) as usize] = key;
        }
    }
}

/// Slightly more rusty version.
pub fn sort2(data: &mut Vec<i32>) {
    for j in 1..data.len() {
        let key = data[j];
        for i in (0..j).rev() {
            if data[i] <= key {
                break;
            }
            data.swap(i + 1, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_output_is_ordered() {
        let mut data = vec![5, 4, 3, 3, 1, 2, 3];
        sort(&mut data);
        assert_eq!(vec![1, 2, 3, 3, 3, 4, 5], data);
    }

    #[test]
    fn test_sort2_output_is_ordered() {
        let mut data = vec![5, 4, 3, 3, 1, 2, 3];
        sort2(&mut data);
        assert_eq!(vec![1, 2, 3, 3, 3, 4, 5], data);
    }
}
