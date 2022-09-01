use std::io;

#[derive(Debug)]
pub struct Instruction {
    pub action: String,
    pub subject: String,
    pub particle: String,
    pub object: String,
}

pub fn get_predicate () -> Option<String> {
    println!("Introduce el predicado a ejecutar");
    let mut predicate = String::new();
    io::stdin().read_line(&mut predicate).expect("Error al obtener tu predicado");
    if validate_predicate(&predicate){
        predicate = predicate.trim().to_string();
        Some(predicate)
    }else{
        None
    }
}

fn validate_predicate(predicate: &String) -> bool {
    let count = predicate.split_whitespace().count();
    if count == 4 {
        true
    } else {
        false
    }
}

pub fn build_instruction_from_predicate(predicate: String) -> Instruction {

    let mut  iterator = predicate.split_whitespace();
    let action = assign_instruction_from_option(iterator.next());
    let subject = assign_instruction_from_option(iterator.next());
    let particle = assign_instruction_from_option(iterator.next());
    let object = assign_instruction_from_option(iterator.next());
    new_instruction(action, subject, particle, object)
}

fn assign_instruction_from_option(option: Option<&str>) -> String{
    match option{
        Some(value) => String::from(value),
        None => String::from(""),
    }
}



pub fn new_instruction(action: String,subject: String,particle: String,object: String,) -> Instruction{
        Instruction {
            action,
            subject,
            particle,
            object,
        }
}