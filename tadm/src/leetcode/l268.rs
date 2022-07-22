// https://leetcode.com/problems/missing-number/

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let max = nums.len();
    let mut identity = [1; 10000];
    nums.iter().for_each(|&n| {
        identity[n as usize] = 0;
    });

    for i in 0..=max {
        if identity[i] != 0 {
            return i as i32;
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
