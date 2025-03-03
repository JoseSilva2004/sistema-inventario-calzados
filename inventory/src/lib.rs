use std::io::{self};
pub struct Item {
    id: u32,
    codigo_inventario: String,
    name: String,
    model: String,
    price: f32,
    quantity: u32,
}

 pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
   pub fn new() -> Inventory {
        Inventory { items: Vec::new() }
    }

    pub fn add_item(&mut self, id: u32, codigo_inventario: String, name: String, model: String, price: f32, quantity: u32) {
        self.items.push(Item { id, codigo_inventario, name, model, price, quantity });
    }

    //Eliminar un calzado por Codigo de inventario
    pub fn eliminar_calzado<F>(&mut self, codigo_inventario: &str, clear_screen: F) 
    where
        F: Fn(),
    {
        clear_screen();
        let initial_count = self.items.len();
        self.items.retain(|item| item.codigo_inventario != codigo_inventario);
        
        if self.items.len() < initial_count {
            println!("El Calzado con código {} fue eliminado correctamente.", codigo_inventario);
        } else {
            println!("El Calzado con código {} no fue encontrado.", codigo_inventario);
        }

        wait_for_keypress();
    }

    /*Eliminar inventario por ID */
    pub fn remove_inventory<F>(&mut self, codigo_inventario: &str, quantity: u32, clear_screen: F)
    where
        F: Fn(),
    {
        clear_screen();
        let item = self.items.iter_mut().find(|item| item.codigo_inventario == codigo_inventario);
        if let Some(item) = item {
            if item.quantity >= quantity {
                item.quantity -= quantity;
                println!("Se han eliminado {} pares del calzado con Referencia {}.", quantity, codigo_inventario);
            } else {
                println!("No hay suficientes pares del calzado con Referencia {} en el inventario.", codigo_inventario);
            }
        } else {
            println!("El calzado con Referencia {} no fue encontrado.", codigo_inventario);
        }

        wait_for_keypress();

    }

    pub fn list_items<F>(&self, clear_screen: F)
    where
        F: Fn(), // F es un closure que no toma argumentos y no devuelve nada
    {
        clear_screen(); // Limpiar la pantalla usando el callback
        println!("\n{:<5} {:<15} {:<15} {:<15} {:<10} {:<18}", "ID", "Código", "Marca", "Modelo", "Precio", "Cantidad en Inventario");
        println!("{}", "-".repeat(90));
        
        for item in &self.items {
            println!("{:<5} {:<15} {:<15} {:<15} {:<10.2} {:<18}", item.id, item.codigo_inventario, item.name, item.model, item.price, item.quantity);
        }
        
        wait_for_keypress();
    }

}

pub fn wait_for_keypress() {
    println!("\nPresione Enter para continuar...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
}