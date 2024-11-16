use std::io::{self, BufRead};

fn main() {
    let stdin=io::stdin();
    let mut iterator=stdin.lock().lines();
    let t:i32=iterator.next().unwrap().unwrap().trim().parse().unwrap();
    for k in 0..t{
        let n:i32=iterator.next().unwrap().unwrap().trim().parse().unwrap();
       
        let mut counter:i32=1;
        if n%2==0{
            for j in 1..n+1{
                print!("{} ",counter);
                if j%2==0{
                    counter=counter+1;
                }
            }
        }
        else{
            if n<27{
                print!("-1");
            }
            else{
                
                let mut v = vec![0; (n+1) as usize];
                let mut orgCounter:i32=0;
                counter=3;
                v[1]=1;
                v[10]=1;
                v[26]=1;
                v[27]=2;
                v[23]=2;
                let sz:usize= (n+1) as usize;
                for i in 1..sz{
                    if i!=1 && i!=10 && i !=23 && i!=26 && i!=27{
                        v[i]=counter;
                        orgCounter=orgCounter+1;
                        if orgCounter%2==0{
                            counter=counter+1;
                            
                        }
                    }
                    print!("{} ",v[i]);
                }
            }
        }
        println!("");
      

        
        
    }

}
