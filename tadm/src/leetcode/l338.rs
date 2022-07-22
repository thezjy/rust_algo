/// https://leetcode.com/problems/counting-bits/
pub fn count_bits(n: i32) -> Vec<i32> {
    let first_eight = [0, 1, 1, 2, 1, 2, 2, 3];

    let mut result = vec![];

    let mut bound = 8;
    (0..=n).for_each(|i| {
        if i >= bound * 2 {
            bound *= 2;
        }

        if i < 8 {
            result.push(first_eight[i as usize]);
        } else {
            result.push(result[(i - bound) as usize] + 1);
        }
    });

    result
}

#[test]
fn test_count_bits() {
    assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    assert_eq!(
        count_bits(18),
        vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2,]
    );
}
