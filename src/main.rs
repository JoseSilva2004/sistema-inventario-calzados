//Sistema de inventario para el área de calzado de la Mr Price de Costa Azul
use inventory::{wait_for_keypress, Inventory};
use std::io;
use std::process::Command;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Fallo al ejecutar el comando");
    } else {
        Command::new("clear")
            .status()
            .expect("Fallo al ejecutar el comando");
    }
}

fn main() {
    let mut inventory = Inventory::new();
    
    loop {
        println!("\nSistema de inventario (Calzados) MrPrice Costa Azul");
        println!("\nOpciones:");
        println!("1. Añadir Calzado");
        println!("2. Eliminar Calzado");
        println!("3. Eliminar inventario por Referencia");
        println!("4. Listar todos los calzados");
        println!("5. Salir del programa");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error al leer la entrada");
        
        match choice.trim() {
            /*Añadir un calzado */

            "1" => {
                let mut id = String::new();
                let mut codigo_inventario = String::new();
                let mut name = String::new();
                let mut model = String::new();
                let mut price = String::new();
                let mut quantity = String::new();
                
                println!("\nIngrese el ID:");
                io::stdin().read_line(&mut id).expect("Error al leer la entrada");

                println!("\n Ingrese el código:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");
                
                println!("\nIngrese la marca:");
                io::stdin().read_line(&mut name).expect("Error al leer la entrada");
                
                println!("\nIngrese el modelo:");
                io::stdin().read_line(&mut model).expect("Error al leer la entrada");

                println!("\nIngrese precio:");
                io::stdin().read_line(&mut price).expect("Error al leer la entrada");
                
                println!("\nIngrese la cantidad en el inventario:");
                io::stdin().read_line(&mut quantity).expect("Error al leer la entrada");

                clear_screen();
                
                /*Se hace la conversion a su tipo de datos correspondiente */
                let id: u32 = id.trim().parse().expect("Por favor, ingrese un número válido"); 
                let quantity: u32 = quantity.trim().parse().expect("Por favor, ingrese un número válido");
                let price : f32 = price.trim().parse().expect("Por favor, ingrese un número válido");

                inventory.add_item(id, codigo_inventario.trim().to_string().to_uppercase(), name.trim().to_string().to_uppercase(), model.trim().to_string().to_uppercase(), price, quantity);
                println!("\n¡Calzado añadido correctamente!.");
                wait_for_keypress();
                
            }
            /*Eliminar un calzado */
            "2" => {
                let mut codigo_inventario = String::new();
                println!("Ingrese codigo de referencia del calzado a eliminar:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");
                
                inventory.eliminar_calzado(&codigo_inventario.trim().to_uppercase(), clear_screen);
            }
            /*Eliminar inventario de un calzado por Referencia */
            "3" => {
                let mut codigo_inventario = String::new();
                let mut quantity = String::new();
                println!("Ingrese el código de referencia del calzado:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");
                
                println!("Ingrese la cantidad a eliminar:");
                io::stdin().read_line(&mut quantity).expect("Error al leer la entrada");
                
                let quantity: u32 = quantity.trim().parse().expect("Por favor, ingrese un número válido");
                inventory.remove_inventory(&codigo_inventario.trim().to_uppercase(), quantity, clear_screen);

            }
            
            /*Listar todos los calzados */
            "4" => inventory.list_items(clear_screen),
            
            /*Salir del programa */
            "5" => break,
            _ => println!("Opción no válida, por favor intente de nuevo."),
        }
    }
}
