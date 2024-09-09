fn main(){
    println!("program to reverse element");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read line");
    let mut num:i32=s.trim().parse().expect("enter valid number");
    let mut rev:i32=0;
    while num!=0 {
        let q=num%10;
        rev=rev*10+q;
        num=num/10;
    }
    println!("reversed value: {}",rev);
    }