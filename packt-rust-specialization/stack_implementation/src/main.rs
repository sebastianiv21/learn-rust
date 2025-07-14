fn main() {
    println!("Please mention the size of the stack");
    let size_stack = get_input();

    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n **** Menu **** \n");
        println!("1. Push\n2. Pop\n3. Display\n4. Size\n5. Exit");
        println!("\nEnter your choice: ");
        let choice = get_input();

        match choice {
            1 => {
                println!("Enter the value to insert: ");
                let item = get_input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("The element which is poped is: {:?}", pop(&mut stack)),
            3 => println!("The elements are {:?}", stack),
            4 => println!("The size of the stack is {}", size(&stack)),
            5 => break,
            _ => println!("Wrong selection, try again"),
        }

        println!("Want to continue? 1 = Yes / 0 = No");
        let status = get_input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }
}

fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val = stack.pop();
    println!("The poped value is {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Can not add more");
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn get_input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let n: u32 = n.trim().parse().expect("Invalid u32");
    n
}
