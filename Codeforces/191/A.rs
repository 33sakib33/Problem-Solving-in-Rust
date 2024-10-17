use std::io::{self, BufRead};
use std::cmp::max;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
  
    let n: i32 = iterator.next().unwrap().unwrap().trim().parse().unwrap();

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 26]; 26];
    
   
    let offset: usize = 'a' as usize;

  
    let mut lines: Vec<String> = Vec::new();

 
    for _ in 0..n {
        let line = iterator.next().unwrap().unwrap();
        lines.push(line.trim().to_string());
    }

    for line in lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
    
        let first_char = chars[0] as usize - offset;
        let second_char = chars[chars.len()-1] as usize - offset; 
        let d: i64 = chars.len() as i64;
        
        for i in 0..26{
            let existing_len: i64 =dp[second_char][i];
            
            if existing_len>0{
                dp[first_char][i]=max(dp[first_char][i],existing_len+d);
            }
            
        }
        dp[first_char][second_char]=max(dp[first_char][second_char],d);
       
         
        
    }
    let mut ans: i64=0;
    for i in 0..26{
        ans=max(ans,dp[i][i]);
    }
    println!("{}",ans);

 
}

