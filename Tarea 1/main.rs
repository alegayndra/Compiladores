mod stack;

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

fn main() {
    pruebas_stack();
}