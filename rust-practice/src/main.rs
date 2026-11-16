use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Tarea {
    titulo: String,
    completada: bool,
    dias_restantes: i32,
}

fn main() {
    match cargar() {
        Ok(mis_tareas) => {
            println!("Num tareas: {}", mis_tareas.len());

            match buscar_por_titulo(&mis_tareas, "Ir a dormir ya de ya !!!") {
                Some(i) => println!("Encontrada en la posición {}", i),
                None => println!("No existe"),
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}

fn cargar() -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    if !std::path::Path::new("tareas.json").exists() {
        return Ok(Vec::new());
    }
    let texto = std::fs::read_to_string("tareas.json")?;
    let lista = serde_json::from_str::<Vec<Tarea>>(&texto)?;
    Ok(lista)
}

#[allow(dead_code)]
fn guardar(lista: &Vec<Tarea>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(lista)?;
    std::fs::write("tareas.json", json)?;
    Ok(())
}

fn buscar_por_titulo(lista: &Vec<Tarea>, titulo: &str) -> Option<usize> {
    for (i, tarea) in lista.iter().enumerate() {
        if tarea.titulo == titulo {
            return Some(i);
        }
    }
    None
}
