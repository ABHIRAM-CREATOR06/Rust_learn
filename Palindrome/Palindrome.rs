fn main(){
    println!("program to find Palindrome ");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed to read line");
    let input=s.trim();
    let size=input.len();
    let mut low=0;
    let mut high=size-1;
    let mut flag=true;
    while low<high {
        if input.chars().nth(low).unwrap()!=input.chars().nth(high).unwrap(){
            flag=false;
            break;
        }
        low+=1;
        high-=1;
    }
    if flag {
        println!("entered text {} is a Palindrome",input);
    }
    else {
        println!("entered Text {} is not palindrome",input);
    }
    }