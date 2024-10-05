fn main(){
    println!("WELCOME TO RUST DSA LEARN PART-1");
    println!("I WILL BE USING MY DSA IN KNOWLEDGE IN C AND WORK IN RUST");
    println!("PUSH,POP,OPERATION IN STACK");
    let mut stack = Vec::new();
    let mut sc = String::new();
    println!("AVAILABLE OPERATIONS: ");
    println!("1. PUSH");
    println!("2. POP");
    println!("3. DISPLAY");
    println!("4. EXIT");
    loop{
        println!("ENTER YOUR CHOICE:");
        std::io::stdin().read_line(&mut sc).expect("Failed to read line");
        let choice: i32 = match sc.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid option entered");
                continue;
            }
        };
        sc.clear();
        match choice{
            1 => push(&mut stack),
            2 => pop(&mut stack),
            3 => display(&stack),
            4 => break,
            _ => println!("invalid option entered"),
        }
    }
}

fn push(stack: &mut Vec<i32>){
    let mut sc = String::new();
    println!("ENTER THE ELEMENT TO BE PUSHED:");
    std::io::stdin().read_line(&mut sc).expect("Failed to read line");
    let element: i32 = match sc.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid option entered");
            return;
        }
    };
    sc.clear();
    stack.push(element);
}

fn pop(stack: &mut Vec<i32>){
    if stack.len()==0{
        println!("UNDERFLOW");
    }
    else{
        if let Some(poped) = stack.pop() {
            println!("POPPED ELEMENT: {}", poped);
        }
    }
}

fn display(stack: &Vec<i32>){
    if stack.len()==0{
        println!("STACK IS EMPTY");
    }
    else{
        for i in 0..stack.len(){
            print!("{} ",stack[i]);
        }
        println!();
    }
}
