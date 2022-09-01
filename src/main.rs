mod cli;
use cli::build_instruction_from_predicate;
use std::collections::HashMap;

fn main() {
    let mut employee_registry: HashMap<String, Vec<String>> = HashMap::new();
    let mut keep_exec = true;

    while keep_exec {
        let predicate = cli::get_predicate();
        //dbg!(predicate);
        match predicate {
            Some(string_predicate) => {
                let instruction = build_instruction_from_predicate(string_predicate);
                if instruction.action.eq("asignar") {
                    println!("Iniciando asignacion de {} {} {}",instruction.subject,instruction.particle,instruction.object);
                    let flag = assign_employee(&mut employee_registry, &instruction.subject, &instruction.object);
                    if flag == false {
                        create_department_assign_employee(&mut employee_registry, instruction.subject, instruction.object);
                    }
                }
                employee_registry = dbg!(employee_registry);

            },
            None => println!("predicado no valido"),
        }
    }
    
}

fn assign_employee(registry: &mut HashMap<String, Vec<String>>,employee: &str, department: &str) -> bool {
    let mut flag = false;
    for (key, val) in registry {
        if key.eq(&department) && flag == false {
            val.push(String::from(employee));
            flag = true;
        }
    }
    flag
}

fn create_department_assign_employee(registry: &mut HashMap<String, Vec<String>>,employee: String, department: String){
    let temp_vec = vec![employee];
    registry.insert(department, temp_vec);
}
