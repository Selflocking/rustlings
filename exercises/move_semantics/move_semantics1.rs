// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// move 进来, 原本的vec是immut的，加上mut标识为可以更改的
// fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
// 等效于下面两行
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
