use db::{Database, Shoe, clear_screen, wait_for_keypress};
use std::io;
use colored::*;

fn main() {
    let db = Database::new("mysql://root:30230054jose@localhost:3306/calzados_mrprice")
        .expect("Error al conectar con la base de datos");

    loop {
        clear_screen();
        println!("\n{}", "╔════════════════════════════════════════════════════════════════╗".bright_blue());
        println!("{}", "║                  SISTEMA DE INVENTARIO (CALZADOS)               ║".bright_red());
        println!("{}", "║                         MrPrice Costa Azul                      ║".bright_red());
        println!("{}", "║                                                                 ║".bright_blue());
      println!("\n{}", "║         **Desarrollado por José Silva (Estudiante UDONE)**      ║".bright_yellow().bold());
        println!("{}", "╠════════════════════════════════════════════════════════════════╣".bright_blue());
        println!("{}", "║ Opciones:                                                      ║".bright_blue());
        println!("{}", "║ 1 - Añadir Calzado                                             ║".bright_green());
        println!("{}", "║ 2 - Registrar tallas para un calzado                           ║".bright_green());
        println!("{}", "║ 3 - Eliminar Calzado                                           ║".bright_green());
        println!("{}", "║ 4 - Eliminar inventario por talla                              ║".bright_green());
        println!("{}", "║ 5 - Listar todos los calzados                                  ║".bright_green());
        println!("{}", "║ 6 - Buscar calzado por código de referencia                    ║".bright_green());
        println!("{}", "║ 7 - Modificar un calzado                                       ║".bright_green());
        println!("{}", "║ 8 - Salir del programa                                         ║".bright_green());
        println!("{}", "╚════════════════════════════════════════════════════════════════╝".bright_blue());
        
        println!("\n{}", "Por favor, seleccione una opción:".bright_blue());
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error al leer la entrada");

        match choice.trim() {
            "1" => {
                clear_screen();

                let mut codigo_inventario = String::new();
                let mut marca = String::new();
                let mut modelo = String::new();
                let mut precio = String::new();
                let mut color = String::new(); 

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

                println!("\nIngrese el color:");
                io::stdin().read_line(&mut color).expect("Error al leer la entrada");

                clear_screen();

                let precio: f32 = precio.trim().parse().expect("Por favor, ingrese un número válido");

                let shoe = Shoe {
                    id: 0, // El ID se genera automáticamente en la base de datos
                    codigo_inventario: codigo_inventario.trim().to_string(),
                    marca: marca.trim().to_string(),
                    modelo: modelo.trim().to_string(),
                    precio,
                    color: color.trim().to_string(), // Nuevo campo: color
                };

                match db.add_shoe(&shoe) {
                    Ok(calzado_id) => {
                        println!("\n¡Calzado añadido correctamente! ID: {}", calzado_id);
                    }
                    Err(err) => {
                        println!("Error al añadir el calzado: {}", err);
                    }
                }

                wait_for_keypress();
            }
            "2" => {
                clear_screen();

                let mut codigo_inventario = String::new();
                println!("Ingrese el código de referencia del calzado:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                // Buscar el calzado para obtener su ID
                match db.find_shoe_by_code(codigo_inventario.trim()) {
                    Ok(Some((shoe, _))) => {
                        loop {
                            clear_screen();
                            let mut talla = String::new();
                            let mut cantidad = String::new();

                            println!("Ingrese la talla (o 'fin' para terminar):");
                            io::stdin().read_line(&mut talla).expect("Error al leer la entrada");

                            if talla.trim().to_lowercase() == "fin" {
                                break;
                            }

                            println!("Ingrese la cantidad para la talla {}:", talla.trim());
                            io::stdin().read_line(&mut cantidad).expect("Error al leer la entrada");

                            let cantidad: u32 = cantidad.trim().parse().expect("Por favor, ingrese un número válido");

                            if let Err(err) = db.add_talla(shoe.id, talla.trim(), cantidad) {
                                println!("Error al registrar la talla: {}", err);
                            } else {
                                println!("Talla {} registrada correctamente.", talla.trim());
                            }

                            wait_for_keypress();
                        }
                    }
                    Ok(None) => {
                        println!("\nEl calzado con código {} no fue encontrado. Por favor, registre el calzado primero.", codigo_inventario.trim());
                        wait_for_keypress();
                    }
                    Err(err) => {
                        println!("Error al buscar el calzado: {}", err);
                        wait_for_keypress();
                    }
                }
            }
            "3" => {
                clear_screen();
                let mut codigo_inventario = String::new();
                println!("Ingrese el código de referencia del calzado a eliminar:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                if let Err(err) = db.remove_shoe(codigo_inventario.trim()) {
                    println!("{}", err);
                }

                wait_for_keypress();
            }
            "4" => {
                clear_screen();
                let mut codigo_inventario = String::new();
                let mut talla = String::new();
                let mut cantidad = String::new();

                println!("Ingrese el código de referencia del calzado:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                clear_screen();

                println!("Ingrese la talla:");
                io::stdin().read_line(&mut talla).expect("Error al leer la entrada");

                clear_screen();

                println!("Ingrese la cantidad a eliminar:");
                io::stdin().read_line(&mut cantidad).expect("Error al leer la entrada");

                clear_screen();

                let cantidad: u32 = cantidad.trim().parse().expect("Por favor, ingrese un número válido");

                if let Err(err) = db.remove_inventory(codigo_inventario.trim(), talla.trim(), cantidad) {
                    println!("{}", err);
                }

                wait_for_keypress();
            }
            "5" => {
                clear_screen();
                match db.list_shoes() {
                    Ok(shoes) => {
                        let mut pagina_actual = 0;
                        let cantidad_por_pagina = 5;
                        let total_paginas = (shoes.len() as f32 / cantidad_por_pagina as f32).ceil() as usize;

                        loop {
                            clear_screen();
                            println!(
                                "\n{} (Página {} de {}):",
                                "Listado de calzados".bold().blue(),
                                (pagina_actual + 1).to_string().green(),
                                total_paginas.to_string().red()
                            );

                            println!(
                                "\n{:<5} {:<15} {:<15} {:<15} {:<10} {:<10}",
                                "ID".bold().green(),
                                "Código".bold().cyan(),
                                "Marca".bold().yellow(),
                                "Modelo".bold().magenta(),
                                "Precio".bold().red(),
                                "Color".bold().blue()
                            );
                            
                            println!(
                                "{}",
                                "-".repeat(70).bright_black().green() // Línea separadora con estilo
                            );

                            // Calcular el rango de calzados a mostrar
                            let inicio = pagina_actual * cantidad_por_pagina;
                            let fin = std::cmp::min(inicio + cantidad_por_pagina, shoes.len());

                            // Mostrar los calzados de la página actual
                            for (shoe, tallas) in &shoes[inicio..fin] {
                                println!(
                                    "{:<5} {:<15} {:<15} {:<15} {:<10} {:<10}",
                                    shoe.id.to_string().green().bold(),
                                    shoe.codigo_inventario.cyan().bold(),
                                    shoe.marca.yellow().bold(),
                                    shoe.modelo.magenta().bold(),
                                    format!("{:.2}", shoe.precio).red(), // Formatear el precio antes de aplicarle el color
                                    shoe.color.blue().bold()
                                );

                                // Mostrar las tallas asociadas al calzado
                                if !tallas.is_empty() {
                                    println!("\n Tallas:");
                                    for talla in tallas {
                                        println!(" - Talla: {}, Cantidad: {}", talla.talla.bold().bright_green(), talla.cantidad.to_string().bold().bright_blue());
                                    }
                                } else {
                                    println!("  No hay tallas registradas para este calzado.");
                                }
                                println!("{}", "-".repeat(70).bright_black().green()); // Línea separadora entre calzados
                            }

                            // Menú de paginación
                            println!("\nOpciones:");
                            println!("1 - Siguiente página");
                            println!("2 - Página anterior");
                            println!("3 - Volver al menú principal");

                            let mut opcion_paginacion = String::new();
                            println!("\nSeleccione una opción:");
                            io::stdin().read_line(&mut opcion_paginacion).expect("Error al leer la entrada");

                            match opcion_paginacion.trim() {
                                "1" => {
                                    if pagina_actual < total_paginas - 1 {
                                        pagina_actual += 1;
                                    } else {
                                        println!("\n{}", "¡Ya estás en la última página.!".bold().bright_red());
                                        wait_for_keypress();
                                    }
                                }
                                "2" => {
                                    if pagina_actual > 0 {
                                        pagina_actual -= 1;
                                    } else {
                                        println!("\n{}","¡Ya estás en la primera página.!".bold().bright_red());
                                        wait_for_keypress();
                                    }
                                }
                                "3" => break,
                                _ => println!("Opción no válida, por favor intente de nuevo."),
                            }
                        }
                    }
                    Err(err) => println!("Error al listar los calzados: {}", err),
                }
            }
            "6" => {
                clear_screen();
                let mut codigo_inventario = String::new();
                println!("Ingrese el código de referencia del calzado a buscar:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                match db.find_shoe_by_code(codigo_inventario.trim()) {
                    Ok(Some((shoe, tallas))) => {
                        // Encabezado de la tabla
                        println!(
                            "\n{:<5} {:<15} {:<15} {:<15} {:<10} {:<10}",
                            "ID", "Código", "Marca", "Modelo", "Precio", "Color"
                        );
                        println!("{}", "-".repeat(70));

                        // Mostrar el calzado en una fila de la tabla
                        println!(
                            "{:<5} {:<15} {:<15} {:<15} {:<10.2} {:<10}",
                            shoe.id, shoe.codigo_inventario, shoe.marca, shoe.modelo, shoe.precio, shoe.color
                        );

                        println!("\n  Tallas:");
                        // Mostrar las tallas asociadas al calzado
                        if !tallas.is_empty() {
                            for talla in tallas {
                                println!("    - Talla: {}, Cantidad: {}", talla.talla, talla.cantidad);
                            }
                        } else {
                            println!("  No hay tallas registradas para este calzado.");
                        }
                        println!("{}", "-".repeat(70)); // Línea separadora
                    }
                    Ok(None) => {
                        println!("\nEl calzado con código {} no fue encontrado.", codigo_inventario.trim().to_uppercase());
                    }
                    Err(err) => {
                        println!("Error al buscar el calzado: {}", err);
                    }
                }
                wait_for_keypress();
            }
            "7" => {
                clear_screen();
                let mut codigo_inventario = String::new();
                println!("Ingrese el código de referencia del calzado a modificar:");
                io::stdin().read_line(&mut codigo_inventario).expect("Error al leer la entrada");

                match db.find_shoe_by_code(codigo_inventario.trim()) {
                    Ok(Some((shoe, tallas))) => {
                        // Mostrar los datos actuales del calzado
                        println!(
                            "\n{:<5} {:<15} {:<15} {:<15} {:<10} {:<10}",
                        "ID", "Código", "Marca", "Modelo", "Precio", "Color"
                        );
                        println!("{}", "-".repeat(70)); // Línea separadora
                        println!(
                            "{:<5} {:<15} {:<15} {:<15} {:<10.2} {:<10}",
                            shoe.id, shoe.codigo_inventario, shoe.marca, shoe.modelo, shoe.precio, shoe.color
                        );

                        // Mostrar las tallas actuales
                        if !tallas.is_empty() {
                            println!("\n  Tallas actuales:");
                            for talla in tallas {
                                println!("    - Talla: {}, Cantidad: {}", talla.talla, talla.cantidad);
                            }
                        } else {
                            println!("\n  No hay tallas registradas para este calzado.");
                        }

                        // Solicitar nuevos datos del calzado
                        let mut nueva_marca = String::new();
                        let mut nuevo_modelo = String::new();
                        let mut nuevo_color = String::new();
                        let mut nuevo_precio = String::new();

                        println!("\nIngrese la nueva marca (deje en blanco para no modificar):");
                        io::stdin().read_line(&mut nueva_marca).expect("Error al leer la entrada");

                        println!("Ingrese el nuevo modelo (deje en blanco para no modificar):");
                        io::stdin().read_line(&mut nuevo_modelo).expect("Error al leer la entrada");

                        println!("Ingrese el nuevo color (deje en blanco para no modificar):");
                        io::stdin().read_line(&mut nuevo_color).expect("Error al leer la entrada");

                        println!("Ingrese el nuevo precio (deje en blanco para no modificar):");
                        io::stdin().read_line(&mut nuevo_precio).expect("Error al leer la entrada");

                        // Actualizar el calzado si se proporcionaron nuevos datos
                        let marca = if nueva_marca.trim().is_empty() {
                            shoe.marca.clone()
                        } else {
                            nueva_marca.trim().to_string()
                        };

                        let modelo = if nuevo_modelo.trim().is_empty() {
                            shoe.modelo.clone()
                        } else {
                            nuevo_modelo.trim().to_string()
                        };

                        let color = if nuevo_color.trim().is_empty() {
                            shoe.color.clone()
                        } else {
                            nuevo_color.trim().to_string()
                        };

                        let precio = if nuevo_precio.trim().is_empty() {
                            shoe.precio
                        } else {
                            nuevo_precio.trim().parse().expect("Por favor, ingrese un número válido")
                        };

                        if let Err(err) = db.update_shoe(shoe.id, &marca, &modelo, &color, precio) {
                            println!("Error al actualizar el calzado: {}", err);
                        } else {
                        println!("\nCalzado actualizado correctamente.");
                        }

                        // Modificar las tallas
                        loop {
                            clear_screen();
                            println!("\nOpciones para modificar tallas:");
                            println!("1 - Agregar una nueva talla");
                            println!("2 - Modificar la cantidad de una talla existente");
                            println!("3 - Eliminar una talla existente");
                            println!("4 - Terminar modificación de tallas");

                            let mut opcion_talla = String::new();
                            println!("\nSeleccione una opción:");
                            io::stdin().read_line(&mut opcion_talla).expect("Error al leer la entrada");

                            match opcion_talla.trim() {
                                "1" => {
                                    let mut nueva_talla = String::new();
                                    let mut cantidad = String::new();

                                    println!("Ingrese la nueva talla:");
                                    io::stdin().read_line(&mut nueva_talla).expect("Error al leer la entrada");

                                    println!("Ingrese la cantidad para la talla {}:", nueva_talla.trim());
                                    io::stdin().read_line(&mut cantidad).expect("Error al leer la entrada");

                                    let cantidad: u32 = cantidad.trim().parse().expect("Por favor, ingrese un número válido");

                                    if let Err(err) = db.add_talla(shoe.id, nueva_talla.trim(), cantidad) {
                                        println!("Error al agregar la talla: {}", err);
                                    } else {
                                        println!("Talla {} agregada correctamente.", nueva_talla.trim());
                                    }
                                }
                                "2" => {
                                    let mut talla = String::new();
                                    let mut nueva_cantidad = String::new();

                                    println!("Ingrese la talla a modificar:");
                                    io::stdin().read_line(&mut talla).expect("Error al leer la entrada");

                                    println!("Ingrese la nueva cantidad para la talla {}:", talla.trim());
                                    io::stdin().read_line(&mut nueva_cantidad).expect("Error al leer la entrada");

                                    let nueva_cantidad: u32 = nueva_cantidad.trim().parse().expect("Por favor, ingrese un número válido");

                                    if let Err(err) = db.update_talla(shoe.id, talla.trim(), nueva_cantidad) {
                                        println!("Error al modificar la talla: {}", err);
                                    } else {
                                        println!("Talla {} modificada correctamente.", talla.trim());
                                    }
                                }
                                "3" => {
                                    let mut talla = String::new();

                                    println!("Ingrese la talla a eliminar:");
                                    io::stdin().read_line(&mut talla).expect("Error al leer la entrada");

                                    if let Err(err) = db.delete_talla(shoe.id, talla.trim()) {
                                        println!("Error: {}", err); // Mostrar el mensaje de error
                                    } else {
                                        println!("Talla {} eliminada correctamente.", talla.trim());
                                    }
                                }
                                "4" => break,
                                _ => println!("Opción no válida, por favor intente de nuevo."),
                            }
                            wait_for_keypress();
                        }
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
            "8" => break,
            _ => println!("Opción no válida, por favor intente de nuevo."),
        }
    }
}

/*Created by dev José Silva */