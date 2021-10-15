use std::vec::Vec;

mod solve;

fn showVec(arr : &mut Vec<i32>) -> () {
    for i in 0..arr.len(){
        println!("{}", arr[i]);
    }
}

fn main() -> () {
    let mut arr : Vec<Vec<i32>> = Vec::new();
    arr.resize(3, Vec::new());
    arr[0].push(1);
    arr[0].push(2);
    arr[1].push(0);
    arr[2].push(0);

    solve::reduction(&mut arr);
}
