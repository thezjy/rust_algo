// https://leetcode.com/problems/valid-anagram/

use std::collections::HashMap;

pub fn is_anagram(mut s: String, mut t: String) -> bool {
    unsafe {
        let mut s = s.as_bytes_mut();
        s.sort();

        let mut t = t.as_bytes_mut();
        t.sort();

        s == t
    }
}

pub fn is_anagram_fast(mut s: String, mut t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut m = HashMap::new();
    s.bytes().for_each(|b| {
        *m.entry(b).or_insert(0) += 1;
    });

    for b in t.bytes() {
        match m.get_mut(&b) {
            Some(count) => {
                if *count == 0 {
                    return false;
                } else {
                    *count -= 1;
                }
            }
            None => {
                return false;
            }
        }
    }

    true
}

pub fn is_anagram_even_faster(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut buckets = [0; 26];
    for (i, j) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
        buckets[(i - b'a') as usize] += 1;
        buckets[(j - b'a') as usize] -= 1;
    }
    buckets == [0; 26]
}

#[test]
fn even_faster() {
    assert!(is_anagram_even_faster(
        "anagram".to_string(),
        "nagaram".to_string()
    ));
}
