mod data_structure;
mod hashtable;
mod tree;

fn main() {
    println!("Hello, world!");
    let a = hashtable::q1::Solution1::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", a);
}
