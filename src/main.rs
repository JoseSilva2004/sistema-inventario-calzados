//Sistema de inventario para el área de calzado de la Mr Price de Costa Azul
use db::{Database, Shoe, clear_screen, wait_for_keypress};
use std::io;

fn main() {
    let db = Database::new("mysql://root:30230054jose@localhost:3306/calzados_mrprice")
        .expect("Error al conectar con la base de datos");

    loop {
        clear_screen();
        println!("\nSistema de inventario (Calzados) MrPrice Costa Azul");
        println!("\nOpciones:");
        println!("1 - Añadir Calzado");
        println!("2 - Eliminar Calzado");
        println!("3 - Eliminar inventario por código de referencia");
        println!("4 - Listar todos los calzados");
        println!("5 - Buscar calzado por código de referencia");
        println!("6 - Salir del programa");

        println!("\nPor favor, seleccione una opción:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error al leer la entrada");

        match choice.trim() {
            "1" => {

                clear_screen();

                let mut codigo_inventario = String::new();
                let mut marca = String::new();
                let mut modelo = String::new();
                let mut precio = String::new();
                let mut cantidad = String::new();

                println!("\nIngrese el código de referencia:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");
                
                clear_screen();

                println!("\nIngrese la marca:");
                io::stdin().read_line(&mut marca).expect("Error al leer la entrada");

                clear_screen();

                println!("\nIngrese el modelo:");
                io::stdin().read_line(&mut modelo).expect("Error al leer la entrada");

                clear_screen();

                println!("\nIngrese el precio:");
                io::stdin().read_line(&mut precio).expect("Error al leer la entrada");

                clear_screen();

                println!("\nIngrese la cantidad en el inventario:");
                io::stdin().read_line(&mut cantidad).expect("Error al leer la entrada");

                clear_screen();

                let precio: f32 = precio.trim().parse().expect("Por favor, ingrese un número válido");
                let cantidad: u32 = cantidad.trim().parse().expect("Por favor, ingrese un número válido");

                let shoe = Shoe {
                    id: 0, // El ID se genera automáticamente en la base de datos
                    codigo_inventario: codigo_inventario.trim().to_string().to_uppercase(),
                    marca: marca.trim().to_string().to_uppercase(),
                    modelo: modelo.trim().to_string().to_uppercase(),
                    precio,
                    cantidad,
                };

                db.add_shoe(&shoe).expect("Error al añadir el calzado");
                println!("\n¡Calzado añadido correctamente!.");

                wait_for_keypress();
            }
            "2" => {

                clear_screen();
                let mut codigo_inventario = String::new();
                println!("Ingrese el código de referencia del calzado a eliminar:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                if let Err(err) = db.remove_shoe(codigo_inventario.trim()) {
                    println!("{}", err);
                }

                wait_for_keypress();
            }
            "3" => {

                clear_screen();
                let mut codigo_inventario = String::new();
                let mut cantidad = String::new();

                println!("Ingrese el código de referencia del calzado:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                clear_screen();

                println!("Ingrese la cantidad a eliminar:");
                io::stdin().read_line(&mut cantidad).expect("Error al leer la entrada");

                clear_screen();

                let cantidad: u32 = cantidad.trim().parse().expect("Por favor, ingrese un número válido");

                if let Err(err) = db.remove_inventory(codigo_inventario.trim(), cantidad) {
                    println!("{}", err);
                }

                wait_for_keypress();
            }
            "4" => {

                clear_screen();
                let shoes = db.list_shoes().expect("Error al listar los calzados");
                println!("\n{:<5} {:<15} {:<15} {:<15} {:<10} {:<18}", "ID", "Código", "Marca", "Modelo", "Precio", "Cantidad en Inventario");
                println!("{}", "-".repeat(90));
                        
                for item in shoes {
                    println!("{:<5} {:<15} {:<15} {:<15} {:<10.2} {:<18}", item.id, item.codigo_inventario, item.marca, item.modelo, item.precio, item.cantidad);
                }
                wait_for_keypress();
            }
            "5" => {
                let mut codigo_inventario = String::new();

                clear_screen();

                println!("Ingrese el código de referencia del calzado a buscar: ");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                match db.find_shoe_by_code(codigo_inventario.trim()) {
                    Ok(Some(shoe)) => {
                        println!(
                            "\n{:<5} {:<15} {:<15} {:<15} {:<10} {:<18}", "ID", "Código", "Marca", "Modelo", "Precio", "Cantidad en Inventario",
                        );
                        println!("{}", "-".repeat(90));
                        println!(
                            "{:<5} {:<15} {:<15} {:<15} {:<10.2} {:<18}",
                            shoe.id, shoe.codigo_inventario, shoe.marca, shoe.modelo, shoe.precio, shoe.cantidad
                        );
                    }
                    Ok(None) => {
                        println!("\nEl calzado con código {} no fue encontrado.", codigo_inventario.trim());
                    }
                    Err(err) => {
                        println!("Error al buscar el calzado: {}", err);
                    }
                }
                wait_for_keypress();
            }
            "6" => break,
            _ => println!("Opción no válida, por favor intente de nuevo."),
        }
    }
}