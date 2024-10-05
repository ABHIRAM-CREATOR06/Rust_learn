use std::io;
use std::collections::VecDeque;

fn main() {
    println!("WELCOME TO RUST DSA LEARN PART-1");
    println!("I WILL BE USING MY DSA KNOWLEDGE IN C AND APPLYING IT IN RUST");
    println!("QUEUE OPERATIONS");
    
    let mut queue = VecDeque::new();
    let mut sc = String::new();
    
    println!("AVAILABLE OPERATIONS: ");
    println!("1. ENQUEUE");
    println!("2. DEQUEUE");
    println!("3. PEEK");
    println!("4. DISPLAY");
    println!("5. EXIT");

    loop {
        println!("ENTER YOUR CHOICE:");
        io::stdin().read_line(&mut sc).expect("Failed to read line");
        let choice: i32 = match sc.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option entered");
                continue;
            }
        };
        sc.clear();
        match choice {
            1 => enqueue(&mut queue),
            2 => dequeue(&mut queue),
            3 => peek(&queue),
            4 => display(&queue),
            5 => break,
            _ => println!("Invalid option entered"),
        }
    }
}

fn enqueue(queue: &mut VecDeque<i32>) {
    let mut sc = String::new();
    println!("ENTER THE ELEMENT TO BE ENQUEUED:");
    io::stdin().read_line(&mut sc).expect("Failed to read line");
    let element: i32 = match sc.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid option entered");
            return;
        }
    };
    queue.push_back(element);
}

fn dequeue(queue: &mut VecDeque<i32>) {
    match queue.pop_front() {
        Some(element) => println!("Dequeued element: {}", element),
        None => println!("Queue is empty"),
    }
}

fn peek(queue: &VecDeque<i32>) {
    match queue.front() {
        Some(element) => println!("Front element: {}", element),
        None => println!("Queue is empty"),
    }
}

fn display(queue: &VecDeque<i32>) {
    if queue.is_empty() {
        println!("Queue is empty");
    } else {
        println!("Queue elements are:");
        for element in queue {
            print!("{} ", element);
        }
        println!();
    }
}