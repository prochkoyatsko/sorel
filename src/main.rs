use std::convert;

struct Tarea {
    titulo: String,
    completada: bool,
    dias_restantes: i32,
}

fn main() {
    let mut mis_tareas: Vec<Tarea> = Vec::new();

    mis_tareas.push(Tarea {
        titulo: String::from("Ir a clase."),
        completada: true,
        dias_restantes: 1,
    });

    mis_tareas.push(Tarea {
        titulo: String::from("Preparar comida."),
        completada: true,
        dias_restantes: 5,
    });

    mis_tareas.push(Tarea {
        titulo: String::from("Salir al trabajo 19:30."),
        completada: false,
        dias_restantes: 0,
    });

    mis_tareas.push(Tarea {
        titulo: String::from("Terminar el fase 1 extra"),
        completada: false,
        dias_restantes: 0,
    });

    let primera = &mis_tareas[0];
    println!("{}", primera.titulo);

    println!("Hay {} tareas en la lista", mis_tareas.len());

    mostrar(&mis_tareas);

    println!("-----------------------");

    comenzar(&mut mis_tareas[0]);
    completar(&mut mis_tareas[2]);

    mostrar(&mis_tareas);

    println!("-----------------------");

    let t = Tarea {
        titulo: String::from("Firmar el documento de prácticas."),
        completada: false,
        dias_restantes: 10,
    };

    archivar(&t);

    println!("-----------------------");

    let total: i32 = contar_urgentes(&mis_tareas);
    println!("Tareas urgentes: {}", total);
}

fn mostrar(mis_tareas: &Vec<Tarea>) {
    for tarea in mis_tareas {
        let res_es_urgente: bool = es_urgente(tarea.dias_restantes);
        println!(
            "Tarea: {} | Completada: {} | Dias restantes: {} | ¿Urgente? {}",
            tarea.titulo, tarea.completada, tarea.dias_restantes, res_es_urgente
        );
    }
}

fn es_urgente(dias_restantes: i32) -> bool {
    dias_restantes <= 2
}

fn completar(tarea: &mut Tarea) {
    tarea.completada = true
}

fn comenzar(tarea: &mut Tarea) {
    tarea.completada = false
}

fn archivar(tarea: &Tarea) {
    println!("Archivada: {} ", tarea.titulo);
}

fn contar_urgentes(lista: &Vec<Tarea>) -> i32 {
    let mut contador: i32 = 0;

    for tarea in lista {
        if es_urgente(tarea.dias_restantes) {
            contador += 1
        }
    }

    contador
}
