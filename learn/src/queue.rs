fn main() {
    println!("WELCOME TO RUST DSA LEARN PART-1");
    println!("I WILL BE USING MY DSA KNOWLEDGE IN C AND WORK IN RUST");
    println!("QUEUE OPERATIONS: PUSH, POP, DISPLAY");
    
    let mut queue = Vec::new();
    let mut sc = String::new();
    
    println!("AVAILABLE OPERATIONS: ");
    println!("1. PUSH");
    println!("2. POP");
    println!("3. DISPLAY");
    println!("4. EXIT");

    loop {
        println!("ENTER YOUR CHOICE:");
        std::io::stdin().read_line(&mut sc).expect("Failed to read line");
        
        let choice: i32 = match sc.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option entered");
                continue;
            }
        };
        
        sc.clear();
        
        match choice {
            1 => push(&mut queue),
            2 => pop(&mut queue),
            3 => display(&queue),
            4 => break,
            _ => println!("Invalid option entered"),
        }
    }
}

fn push(queue: &mut Vec<i32>) {
    let mut sc = String::new();
    println!("ENTER THE ELEMENT TO BE PUSHED:");
    
    std::io::stdin().read_line(&mut sc).expect("Failed to read line");
    let element: i32 = match sc.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid option entered");
            return;
        }
    };
    
    queue.push(element);
}
fn pop(queue: &mut Vec<i32>) {
    if queue.is_empty() {
        println!("UNDERFLOW");
    } else {
        let popped = queue.remove(0); // Directly remove and get the element
        println!("Popped element: {}", popped);
    }
}
fn display(queue: &Vec<i32>) {
    if queue.is_empty() {
        println!("QUEUE IS EMPTY");
    } else {
        for i in 0..queue.len() {
            print!("{} ", queue[i]);
        }
        println!();
    }
}

