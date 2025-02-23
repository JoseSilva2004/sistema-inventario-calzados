//Sistema de inventario para el área de calzado de la Mr Price de Costa Azul
use  inventory::Inventory;
use std::io;

fn main() {
    let mut inventory = Inventory::new();
    
    loop {
        println!("\nOpciones:");
        println!("1. Añadir Calzado");
        println!("2. Eliminar Calzado");
        println!("3. Eliminar inventario por ID");
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
                
                /*Se hace la conversion a su tipo de datos correspondiente */
                let id: u32 = id.trim().parse().expect("Por favor, ingrese un número válido"); 
                let quantity: u32 = quantity.trim().parse().expect("Por favor, ingrese un número válido");
                let price : f32 = price.trim().parse().expect("Por favor, ingrese un número válido");

                inventory.add_item(id, codigo_inventario.trim().to_string(), name.trim().to_string(), model.trim().to_string(), price, quantity);
                println!("\n¡Calzado añadido correctamente!.");
            }
            /*Eliminar un calzado */
            "2" => {
                let mut id = String::new();
                println!("Ingrese ID del ítem a eliminar:");
                io::stdin().read_line(&mut id).expect("Error al leer la entrada");
                
                let id: u32 = id.trim().parse().expect("Por favor, ingrese un número válido");
                inventory.remove_item(id);
            }
            /*Eliminar inventario de un calzado por ID */
            "3" => {
                let mut id = String::new();
                let mut quantity = String::new();
                println!("Ingrese el ID del calzado:");
                io::stdin().read_line(&mut id).expect("Error al leer la entrada");
                
                println!("Ingrese la cantidad a eliminar:");
                io::stdin().read_line(&mut quantity).expect("Error al leer la entrada");
                
                let id: u32 = id.trim().parse().expect("Por favor, ingrese un número válido");
                let quantity: u32 = quantity.trim().parse().expect("Por favor, ingrese un número válido");
                inventory.remove_inventory(id, quantity);
            }
            /*Listar todos los calzados */
            "4" => inventory.list_items(),

            /*Salir del programa */
            "5" => break,
            _ => println!("Opción no válida, por favor intente de nuevo."),
        }
    }
}
