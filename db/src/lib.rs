use mysql::*;
use mysql::prelude::*;
use std::error::Error;
use std::process::Command;
use std::io;

#[derive(Debug, FromRow)]
pub struct Shoe {
    pub id: u32,
    pub codigo_inventario: String,
    pub marca: String,
    pub modelo: String,
    pub precio: f32,
    pub cantidad: u32,
}

//Funcion para limpiar la pantalla
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

//Funcion para esperar a que el usuario presione una tecla
pub fn wait_for_keypress() {
    println!("\nPresione Enter para continuar...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
}

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        let pool = Pool::new(url)?;
        Ok(Database { pool })
    }

    pub fn add_shoe(&self, shoe: &Shoe) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO calzados (codigo_inventario, marca, modelo, precio, cantidad) VALUES (?, ?, ?, ?, ?)",
            (
                &shoe.codigo_inventario,
                &shoe.marca,
                &shoe.modelo,
                &shoe.precio,
                &shoe.cantidad,
            ),
        )?;
        Ok(())
    }

    pub fn remove_shoe(&self, codigo_inventario: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
    
        // Verificar si el calzado existe
        let exists: Option<u32> = conn.exec_first(
            "SELECT id FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;
    
        if exists.is_none() {
            return Err(format!("\nEl calzado con código {} no fue encontrado.", codigo_inventario).into());
        }
    
        // Eliminar el calzado
        conn.exec_drop(
            "DELETE FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;
    
        println!("El calzado con código {} fue eliminado correctamente.", codigo_inventario);
        
        Ok(())
        
    }

    pub fn remove_inventory(&self, codigo_inventario: &str, cantidad: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
    
        // Verificar si el calzado existe
        let exists: Option<u32> = conn.exec_first(
            "SELECT id FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;
    
        if exists.is_none() {

            return Err(format!("El calzado con código {} no fue encontrado.", codigo_inventario.to_uppercase()).into());

        }
    
        // Verificar si hay suficiente cantidad en el inventario
        let current_quantity: Option<u32> = conn.exec_first(
            "SELECT cantidad FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;
    
        if let Some(current_quantity) = current_quantity {

            if current_quantity < cantidad {
                return Err(format!(
                    "No hay suficientes pares del calzado con código {}. Cantidad actual: {}",
                    codigo_inventario.to_uppercase(), current_quantity
                ).into());
            }
        }
    
        // Actualizar el inventario
        conn.exec_drop(
            "UPDATE calzados SET cantidad = cantidad - ? WHERE codigo_inventario = ?",
            (cantidad, codigo_inventario.to_uppercase()),
        )?;
    
        println!("Se han eliminado {} pares del calzado con código {}.", cantidad, codigo_inventario.to_uppercase());

        Ok(())
    }

    pub fn list_shoes(&self) -> Result<Vec<Shoe>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        let shoes = conn.query_map(
            "SELECT id, codigo_inventario, marca, modelo, precio, cantidad FROM calzados",
            |(id, codigo_inventario, marca, modelo, precio, cantidad)| Shoe {
                id,
                codigo_inventario,
                marca,
                modelo,
                precio,
                cantidad,
            },
        )?;

        Ok(shoes)
    }

    pub fn find_shoe_by_code(&self, codigo_inventario: &str) -> Result<Option<Shoe>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        let query = "SELECT id, codigo_inventario, marca, modelo, precio, cantidad FROM calzados WHERE codigo_inventario = ?";
        let shoe: Option<Shoe> = conn.exec_first(query, (codigo_inventario,))?;

        Ok(shoe)
    }
}