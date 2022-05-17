use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        HashSet::<i32>::from_iter(nums1.into_iter())
            .intersection(&HashSet::from_iter(nums2.into_iter()))
            .into_iter()
            .map(|x| *x)
            .collect()
    }
}
