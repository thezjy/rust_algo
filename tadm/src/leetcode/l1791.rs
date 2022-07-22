pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let p1 = &edges[0];
    let p2 = &edges[1];
    let a = p1[0];
    let b = p1[1];
    let c = p2[0];
    let d = p2[1];

    if a == c || a == d {
        a
    } else {
        b
    }
}
