use std::vec::Vec;

#[path="graph.rs"] mod graph;

fn bfs(adj : &mut Vec<Vec<i32>>, src : usize, targ : usize, disallow : (usize, usize)) -> i32 {
    /*
    Min dist from targ to src not using disallow
    */
    return 0;
}

pub fn reduction(adj : &mut Vec<Vec<i32>>) -> () {
    //Just testing stuff out
    for i in (0..adj.len() - 1){
        for j in (0..adj[i].len()){
            println!("Edge from {} to {}", i, adj[i][j]);
        }
    }
}


pub fn closure(adj : &mut Vec<Vec<i32>>) -> () {
    return;
}