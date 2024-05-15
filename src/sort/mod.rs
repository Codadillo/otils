use crate::ObliviousOps;

mod bitonic;
use bitonic::parallel_bitonic_sort;

pub fn osort<T: ObliviousOps + Send>(list: &mut [T], threads: u8) {
    parallel_bitonic_sort(list, 1, threads);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(slice: &[T]) -> bool {
        slice.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_sort() {
        let mut a: [i32; 4] = [3, 1, 2, 4];
        osort(&mut a, 2);
        assert!(is_sorted(&a));
    }
}