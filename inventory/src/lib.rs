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

    pub fn remove_item(&mut self, id: u32) {
        self.items.retain(|item| item.id != id);
        
        if self.items.iter().find(|item| item.id == id).is_none() {
            println!("El ítem con ID {} fue eliminado correctamente.", id);
        } else {
            println!("El ítem con ID {} no fue encontrado.", id);
        }
    }

    /*Eliminar inventario por ID */
    pub fn remove_inventory(&mut self, id: u32, quantity: u32) {
        let item = self.items.iter_mut().find(|item| item.id == id);
        if let Some(item) = item {
            if item.quantity >= quantity {
                item.quantity -= quantity;
                println!("Se han eliminado {} pares del calzado con ID {}.", quantity, id);
            } else {
                println!("No hay suficientes pares del calzado con ID {} en el inventario.", id);
            }
        } else {
            println!("El calzado con ID {} no fue encontrado.", id);
        }
    }

    pub fn list_items(&self) {
        for item in &self.items {
            println!("------------------------------------");
            println!("ID: {}, \nCodigo: {} \nMarca: {}, \nModelo: {}, \nPrecio: {:.2} $, \nCantidad en inventario: {}", item.id, item.codigo_inventario, item.name, item.model, item.price, item.quantity);
            println!("------------------------------------");
        }
    }
}