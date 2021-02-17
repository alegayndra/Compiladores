mod stack;
mod queue;

fn pruebas_stack() {
    println!("\n----- Iniciando pruebas Stack -----\n");

    let valores: [i32; stack::STACK_SIZE] = [0; 500];
    
    let mut stack_local = stack::Stack::new(valores);
    stack_local.push(32);
    stack_local.push(3);
    stack_local.push(100);
    stack_local.show_values();
    stack_local.pop();
    stack_local.show_values();
    println!("front: {}", stack_local.front());
    println!("empty: {}", stack_local.is_empty());
    stack_local.pop();
    stack_local.pop();
    println!("empty: {}", stack_local.is_empty());

    println!("\n----- Terminando pruebas Stack -----\n");
}

fn pruebas_queue() {
    println!("\n----- Iniciando pruebas Queue -----\n");

    let valores: [i32; queue::QUEUE_SIZE] = [0; 500];
    
    let mut queue_local = queue::Queue::new(valores);
    queue_local.push(32);
    queue_local.push(3);
    queue_local.push(100);
    queue_local.show_values();
    queue_local.pop();
    queue_local.show_values();
    println!("front: {}", queue_local.front());
    println!("empty: {}", queue_local.is_empty());
    queue_local.pop();
    queue_local.pop();
    println!("empty: {}", queue_local.is_empty());

    println!("\n----- Terminando pruebas Queue -----\n");
}

fn main() {
    pruebas_stack();
    pruebas_queue();
}