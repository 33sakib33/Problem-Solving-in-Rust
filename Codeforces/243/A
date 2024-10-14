
use std::io::{self, BufRead};
use std::collections::HashSet;



 
fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
     iterator.next().unwrap().unwrap();
    let numbers: Vec<i32> = iterator.next().unwrap().unwrap().trim().split_whitespace().map(|s| s.parse().expect("Parse error")).collect();
    let mut finalAns = HashSet::new();
    let mut contextWindow= HashSet::new();
    
    for x in numbers{
        finalAns.insert(x);
        let mut temp = HashSet::new();
        temp.insert(x);
        for y in &contextWindow{
            finalAns.insert(y|x);
            temp.insert(y|x);
        }
        contextWindow=temp;
    }
  
    println!("{}",finalAns.len());
}
