mod stack;
mod queue;
mod hash;

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

fn pruebas_hash() {
    println!("\n----- Iniciando pruebas Hash -----\n");

    let valores: [i32; queue::QUEUE_SIZE] = [-1; 500];
    let mut hash_local = hash::Hash::new(valores);

    hash_local.add("aaaa", 2);
    hash_local.add("bbbb", 2);
    hash_local.add("aaaa", 2);
    hash_local.show_values();
    hash_local.add("a", 11);
    println!("{}", hash_local.find("aaaa"));
    println!("{}", hash_local.find("b"));
    hash_local.erase("aaaa");
    hash_local.show_values();

    println!("\n----- Final pruebas Hash -----\n");
}

fn main() {
    pruebas_stack();
    pruebas_queue();
    pruebas_hash();
}