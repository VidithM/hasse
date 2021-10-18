use std::vec::Vec;
use std::str::FromStr;
use std::str::from_utf8;
use std::io::{self, Read};

#[path="graph.rs"] mod graph;
#[path="gui.rs"] mod gui;


fn prompt(stdin : &std::io::Stdin, msg : &str, ln : &mut String) -> () {
    println!("{}", msg);
    stdin.read_line(ln);
    let len = ln.trim_end_matches(&['\r', '\n'][..]).len();
    ln.truncate(len);
}

fn main() -> () {
    let mut ln = String::new();
    let mut stdin = io::stdin();

    let mut adj : Vec<Vec<i32>> = Vec::new();

    let mut n : i32 = -1;
    let mut m : i32 = -1;
    let mut dir : bool = false;
    prompt(&stdin, "Enter number of vertices:", &mut ln);

    n = i32::from_str(&ln).unwrap();
    adj.resize(n as usize, Vec::new());

    prompt(&stdin, "Enter number of edges:", &mut ln);

    m = i32::from_str(&ln).unwrap();

    prompt(&stdin, "Is graph directed? (Y/N)", &mut ln);

    if ln == "Y" {
        dir = true;
    } 

    for i in 0..m {
        prompt(&stdin, &mut format!("Enter edge {}", i + 1), &mut ln);
        let toks : Vec<&str> = ln.split(' ').collect();
        let u = i32::from_str(toks[0]).unwrap();
        let v = i32::from_str(toks[1]).unwrap();
        adj[u as usize].push(v);
        if !dir {
            adj[v as usize].push(u);
        }
    }

    let g = graph::Graph {
        adj : adj,
        dir : dir,
    };
}
