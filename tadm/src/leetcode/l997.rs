const MAX: u16 = 1001;

/// https://leetcode.com/problems/find-the-town-judge/
pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut in_degree = [0; MAX as usize];
    let mut out_degree = [0; MAX as usize];

    trust.iter().for_each(|pair| {
        let from = pair[0];
        let to = pair[1];

        out_degree[from as usize] += 1;
        in_degree[to as usize] += 1;
    });

    let mut judge = -1;
    let mut judge_count = 0;
    let judge_in_degree = n - 1;
    let judge_out_degree = 0;
    for i in 1..=n {
        if in_degree[i as usize] == judge_in_degree && out_degree[i as usize] == judge_out_degree {
            judge = i;
            judge_count += 1;
            if (judge_count > 1) {
                break;
            }
        }
    }

    if judge_count == 1 {
        judge
    } else {
        -1
    }
}
