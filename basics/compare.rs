use std::io;
fn main() {
    println!("enter a program test 2 numbers");
    let  mut sc= String::new();
    println!("enter  first number= ");
    std::io::stdin().read_line(&mut sc).expect("failed to read line");
    let  a: i32 = sc.trim().parse().expect("please type a number");
    println!("enter second number= ");
    std::io::stdin().read_line(&mut sc).expect("failed to read line");
    let  b: i32 = sc.trim().parse().expect("please type a number");
    if  a==b{
        println!("both numbers {} {} are equal",a,b);
    }
    else if a!=b {
        if a>b{
            println!("first number {} greater",a);
        }
        else
        {
            println!("second number {} greater",b);
        }
    }
    else{
        println!("error");
    }
}

