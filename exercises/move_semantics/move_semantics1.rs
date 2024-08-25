// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66]; // vec0 owner del vec

    let vec1 = fill_vec(vec0);  // ownership v0 -> vec (param)

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // ownership vec (param) -> vec (local variable)

    vec.push(88);

    vec
}
