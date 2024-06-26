use crate::ObliviousOps;
use std::marker;

mod bitonic;
use bitonic::parallel_bitonic_sort;

pub fn osort<T: ObliviousOps + marker::Send>(list: &mut [T], threads: u8) {
    parallel_bitonic_sort(list, true, threads);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_sort() {
        let mut a: Vec<i32> = (0..128).rev().collect();
        osort(&mut a[..], 2);
        assert!(is_sorted(&a));
    }
}
