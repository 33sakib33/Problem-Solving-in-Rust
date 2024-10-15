
use std::io::{self, BufRead};
use std::cmp::max;


 
fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
   
    let t: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let conds: Vec<i64> = iterator.next().unwrap().unwrap().trim().trim().split_whitespace().map(|s| s.parse().expect("Parse error")).collect();
      
        let  numbers: Vec<i64> = iterator.next().unwrap().unwrap().trim().trim().split_whitespace().map(|s| s.parse().expect("Parse error")).collect();
        let   max_val= numbers.iter().max().unwrap();
      
       let   sum: i64= numbers.iter().sum();
       let ans=max((sum+conds[1]-1)/conds[1],*max_val);
       
       println!("{}",ans);
        
       
    }
    
}
