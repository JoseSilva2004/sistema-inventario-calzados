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
    pub color: String, 
}

#[derive(Debug, FromRow)]
pub struct Talla {
    pub id: u32,
    pub calzado_id: u32,
    pub talla: String,
    pub cantidad: u32,
}

// Función para limpiar la pantalla
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

// Función para esperar a que el usuario presione una tecla
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

    // Registrar un calzado
    pub fn add_shoe(&self, shoe: &Shoe) -> Result<u32, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO calzados (codigo_inventario, marca, modelo, precio, color) VALUES (?, ?, ?, ?, ?)",
            (
                &shoe.codigo_inventario.to_uppercase(),
                &shoe.marca.to_uppercase(),
                &shoe.modelo.to_uppercase(),
                &shoe.precio,
                &shoe.color.to_uppercase(), // Nuevo campo: color
            ),
        )?;

        // Obtener el ID del calzado recién insertado
        let calzado_id: u32 = conn.exec_first(
            "SELECT id FROM calzados WHERE codigo_inventario = ?",
            (&shoe.codigo_inventario.to_uppercase(),),
        )?.ok_or("No se pudo obtener el ID del calzado")?;

        Ok(calzado_id)
    }

    // Registrar una talla asociada a un calzado
    pub fn add_talla(&self, calzado_id: u32, talla: &str, cantidad: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO tallas (calzado_id, talla, cantidad) VALUES (?, ?, ?)",
            (calzado_id, talla.to_uppercase(), cantidad),
        )?;
        Ok(())
    }

    // Eliminar un calzado y sus tallas
    pub fn remove_shoe(&self, codigo_inventario: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        // Verificar si el calzado existe
        let exists: Option<u32> = conn.exec_first(
            "SELECT id FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;

        if exists.is_none() {
            return Err(format!("\nEl calzado con código {} no fue encontrado.", codigo_inventario.to_uppercase()).into());
        }

        // Eliminar el calzado (las tallas se eliminan automáticamente debido a ON DELETE CASCADE)
        conn.exec_drop(
            "DELETE FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;

        println!("El calzado con código {} fue eliminado correctamente.", codigo_inventario.to_uppercase());
        Ok(())
    }

    // Eliminar inventario de una talla específica
    pub fn remove_inventory(&self, codigo_inventario: &str, talla: &str, cantidad: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        // Verificar si el calzado existe
        let calzado_id: Option<u32> = conn.exec_first(
            "SELECT id FROM calzados WHERE codigo_inventario = ?",
            (codigo_inventario.to_uppercase(),),
        )?;

        if calzado_id.is_none() {
            return Err(format!("El calzado con código {} no fue encontrado.", codigo_inventario.to_uppercase()).into());
        }

        // Verificar si hay suficiente cantidad en el inventario
        let current_quantity: Option<u32> = conn.exec_first(
            "SELECT cantidad FROM tallas WHERE calzado_id = ? AND talla = ?",
            (calzado_id, talla),
        )?;

        if let Some(current_quantity) = current_quantity {
            if current_quantity < cantidad {
                return Err(format!(
                    "No hay suficientes pares del calzado con código {} y talla {}. Cantidad actual: {}",
                    codigo_inventario.to_uppercase(), talla, current_quantity
                ).into());
            }
        } else {
            return Err(format!(
                "No se encontró la talla {} para el calzado con código {}.",
                talla, codigo_inventario.to_uppercase()
            ).into());
        }

        // Actualizar el inventario
        conn.exec_drop(
            "UPDATE tallas SET cantidad = cantidad - ? WHERE calzado_id = ? AND talla = ?",
            (cantidad, calzado_id, talla),
        )?;

        println!("Se han eliminado {} pares del calzado con código {} y talla {}.", cantidad, codigo_inventario, talla);
        Ok(())
    }

    // Listar todos los calzados con sus tallas
    pub fn list_shoes(&self) -> Result<Vec<(Shoe, Vec<Talla>)>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        // Obtener todos los calzados
        let shoes: Vec<Shoe> = conn.query_map(
            "SELECT id, codigo_inventario, marca, modelo, precio, color FROM calzados", // Incluir el campo color
            |(id, codigo_inventario, marca, modelo, precio, color)| Shoe {
                id,
                codigo_inventario,
                marca,
                modelo,
                precio,
                color, // Nuevo campo: color
            },
        )?;

        // Obtener las tallas para cada calzado
        let mut result = Vec::new();
        for shoe in shoes {
            let tallas: Vec<Talla> = conn.exec_map(
                "SELECT id, calzado_id, talla, cantidad FROM tallas WHERE calzado_id = ?",
                (shoe.id,),
                |(id, calzado_id, talla, cantidad)| Talla {
                    id,
                    calzado_id,
                    talla,
                    cantidad,
                },
            )?;
            result.push((shoe, tallas));
        }

        Ok(result)
    }

    // Buscar un calzado por código de referencia
    pub fn find_shoe_by_code(&self, codigo_inventario: &str) -> Result<Option<(Shoe, Vec<Talla>)>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        // Obtener el calzado
        let shoe: Option<Shoe> = conn.exec_first(
            "SELECT id, codigo_inventario, marca, modelo, precio, color FROM calzados WHERE codigo_inventario = ?", // Incluir el campo color
            (codigo_inventario.to_uppercase(),),
        )?;

        if let Some(shoe) = shoe {
            // Obtener las tallas del calzado
            let tallas: Vec<Talla> = conn.exec_map(
                "SELECT id, calzado_id, talla, cantidad FROM tallas WHERE calzado_id = ?",
                (shoe.id,),
                |(id, calzado_id, talla, cantidad)| Talla {
                    id,
                    calzado_id,
                    talla,
                    cantidad,
                },
            )?;
            Ok(Some((shoe, tallas)))
        } else {
            Ok(None)
        }
    }

     // Actualizar los datos de un calzado
    pub fn update_shoe(&self, id: u32, marca: &str, modelo: &str, color: &str, precio: f32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "UPDATE calzados SET marca = ?, modelo = ?, color = ?, precio = ? WHERE id = ?",
            (marca.to_uppercase(), modelo.to_uppercase(), color.to_uppercase(), precio, id),
        )?;
        Ok(())
    }

    // Actualizar una talla específica de un calzado
    pub fn update_talla(&self, calzado_id: u32, talla: &str, nueva_cantidad: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "UPDATE tallas SET cantidad = ? WHERE calzado_id = ? AND talla = ?",
            (nueva_cantidad, calzado_id, talla.to_uppercase()),
        )?;
        Ok(())
    }

    // Eliminar una talla específica de un calzado
    pub fn delete_talla(&self, calzado_id: u32, talla: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;

        // Verificar si la talla existe
        let exists: Option<u32> = conn.exec_first(
            "SELECT id FROM tallas WHERE calzado_id = ? AND talla = ?",
            (calzado_id, talla.to_uppercase()),
        )?;

        if exists.is_none() {
            return Err(format!("La talla {} no está registrada para este calzado.", talla).into());
        }

        // Eliminar la talla
        conn.exec_drop(
            "DELETE FROM tallas WHERE calzado_id = ? AND talla = ?",
            (calzado_id, talla.to_uppercase()),
        )?;

        Ok(())
    }

    // Agregar una nueva talla a un calzado
    pub fn agregar_tallas(&self, calzado_id: u32, talla: &str, cantidad: u32) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO tallas (calzado_id, talla, cantidad) VALUES (?, ?, ?)",
            (calzado_id, talla.to_uppercase(), cantidad),
        )?;
        Ok(())
    }
}