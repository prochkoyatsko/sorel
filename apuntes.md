# Apuntes de Rust (proyecto tareas)

Proyecto: gestor de tareas en Rust, con idea de pasarlo luego a Tauri y, a largo plazo, un asistente personal local.
Ruta del proyecto: `D:\proyectos\personales\sorel\task`

## Dónde me quedé

- Fase 0: terminada.
- Fase 1: terminada (struct, Vec, ownership y préstamo con `&`).
- Refuerzo de ownership (`&mut`, conflicto de préstamos, mover vs prestar, `contar_urgentes`): terminado.
- **Siguiente: Fase 2** (guardar y cargar tareas en JSON). Aún no empezada.

## Entorno instalado

- Rust instalado en Windows en `D:\dev\rust` (variables `RUSTUP_HOME` y `CARGO_HOME`).
- Build Tools de C++ en `D:\dev\BuildTools`.
- Comprobar que funciona: `rustc --version` y `cargo --version` (en PowerShell, no en WSL).
- Comandos básicos:
  - `cargo new nombre`: crea un proyecto.
  - `cargo run`: compila y ejecuta.
  - `cargo fmt`: formatea el código.
- Editor: VS Code con la extensión rust-analyzer. Hay que abrir la carpeta que contiene `Cargo.toml`.
- Prettier no sirve para Rust. El formateador es `rustfmt`.

## Fase 0: lo básico

**Variables.** Son inmutables por defecto. Para poder cambiarlas se usa `mut`.

```rust
let nombre = "Alex";               // no se puede cambiar
let mut contador = 0;              // sí se puede cambiar
contador = contador + 1;
```

**Tipos.** Rust infiere el tipo en variables, pero en los parámetros de funciones es obligatorio escribirlo.

| Tipo | Qué es |
|---|---|
| `i32` | entero con signo (admite negativos) |
| `u32` | entero sin signo (solo >= 0) |
| `f64` | decimal |
| `bool` | `true` o `false` |
| `&str` | texto fijo, prestado |
| `String` | texto propio, que puede crecer |

`dias_restantes: i32` son dos cosas: el **nombre** (`dias_restantes`) y el **tipo** (`i32`), separados por `:`.

**Funciones.** El tipo que devuelven va con `->`. La última línea **sin `;`** es lo que devuelve.

```rust
fn es_urgente(dias_restantes: i32) -> bool {
    dias_restantes <= 2
}
```

Una comparación como `<= 2` ya produce un `bool`, no hace falta `if`.

**Llamar a una función:** `es_urgente(1)`. El resultado se guarda en una variable o se imprime.

**Imprimir:** `println!("Hola {}", nombre);`. El `!` indica que es una macro. Cada `{}` se rellena con un valor, en orden.

**Shadowing.** Repetir `let` con el mismo nombre crea una variable nueva que tapa a la anterior. Es legal, pero conviene usar nombres distintos para no confundirse.

## Fase 1: struct, Vec y ownership

**struct.** Agrupa datos con tipos fijos (como un objeto de JS). Define **una sola** tarea.

```rust
struct Tarea {
    titulo: String,
    completada: bool,
    dias_restantes: i32,
}
```

Hay que definirlo **fuera de `main`**. Si está dentro, las demás funciones no lo ven (ámbito o *scope*).

**Vec.** Lista que crece (como un array de JS). Para varias tareas se usa `Vec<Tarea>`.

```rust
let mut mis_tareas: Vec<Tarea> = Vec::new();
mis_tareas.push(Tarea { titulo: String::from("A"), completada: false, dias_restantes: 3 });
println!("{}", mis_tareas.len());
```

**Tipo vs variable.** `Tarea` (mayúscula) es la plantilla, sin datos. `mis_tareas` es la variable con los datos. Si quieres recorrer o pasar la lista, usas la variable, no el tipo.

**Ownership (propiedad).** Cada valor tiene un único dueño. Si se lo das a otro, se **mueve** y ya no es tuyo:

```rust
let t = Tarea { ... };
mis_tareas.push(t);          // t se movió a la lista
println!("{}", t.titulo);    // ERROR: borrow of moved value
```

Formas de evitarlo: usar `t` antes del `push`, meter una copia con `.clone()` (requiere que `Tarea` se pueda clonar), o no usar variable intermedia.

**Préstamo con `&`.** Para mirar un valor sin quedártelo se presta. Para modificarlo prestado se usa `&mut`.

```rust
fn mostrar(lista: &Vec<Tarea>) {
    for tarea in lista {
        println!("{}", tarea.titulo);
    }
}

mostrar(&mis_tareas);        // se presta, main sigue siendo la dueña
```

- El tipo del parámetro es `&Vec<Tarea>`: el tipo de la lista, prestado.
- La forma del bucle es `for ELEMENTO in LISTA`.
- El elemento se nombra en singular (`tarea`) para no mezclarlo con la lista.
- `{}` no sabe mostrar un struct entero: se imprimen sus campos (`tarea.titulo`).

**Dato calculado vs guardado.** Lo urgente no se guarda en el struct, se calcula a partir de `dias_restantes` con `es_urgente`. Guardarlo duplicaría el dato.

## Código final de la fase 1

```rust
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

    println!("Hay {} tareas en la lista", mis_tareas.len());
    mostrar(&mis_tareas);
}

fn mostrar(mis_tareas: &Vec<Tarea>) {
    for tarea in mis_tareas {
        let res_es_urgente = es_urgente(tarea.dias_restantes);
        println!(
            "Tarea: {} | Completada: {} | Dias restantes: {} | ¿Urgente? {}",
            tarea.titulo, tarea.completada, tarea.dias_restantes, res_es_urgente
        );
    }
}

fn es_urgente(dias_restantes: i32) -> bool {
    dias_restantes <= 2
}
```

## Refuerzo de ownership

**Las tres formas de pasar un valor a una función** (como un libro):

| Cómo se escribe | En la vida real | Qué pasa |
|---|---|---|
| `archivar(t)` | Regalas el libro | Ya no lo tienes: si lo usas después, error `moved` |
| `archivar(&t)` | Lo prestas para leer | Sigues siendo el dueño |
| `archivar(&mut t)` | Lo prestas para que lo cambien | Sigues siendo el dueño |

- **Dueño** = la variable que guarda el dato.
- Regla práctica: pedir siempre el permiso mínimo. Si solo se lee, `&`; si se cambia, `&mut`; si la función se lo queda (por ejemplo para guardarlo en otra lista), sin `&`.

**`&mut`.** Hacen falta dos cosas: el parámetro con `&mut` y que el dueño sea `mut`.

```rust
fn completar(tarea: &mut Tarea) {
    tarea.completada = true;
}

let mut mis_tareas: Vec<Tarea> = Vec::new();
completar(&mut mis_tareas[2]);
```

- Si el dueño no es `mut`: `cannot borrow as mutable`. La mutabilidad se declara en el dueño.
- Con un struct prestado no hace falta `*` para cambiar un campo. Con un número prestado sí (`*n = *n + 1`).

**Regla de oro de los préstamos.** En un momento dado puede haber o muchos préstamos `&` (solo lectura), o un único `&mut`, pero nunca los dos a la vez.

```rust
let primera = &mis_tareas[0];   // préstamo de lectura abierto
mis_tareas.push(...);           // ERROR: quiere modificar mientras hay un lector
println!("{}", primera.titulo); // aquí se usa el préstamo
```

Un préstamo se cierra cuando deja de usarse. Si se usa `primera` antes del `push`, compila. Rust lo prohíbe porque un `push` puede reubicar la memoria del `Vec`, y `primera` apuntaría a una dirección que ya no existe.

**Nombres que no hay que mezclar:**

| Nombre | Qué es | Dónde existe |
|---|---|---|
| `Tarea` | El tipo (plantilla) | Todo el archivo |
| `mis_tareas` | La lista con los datos | Solo en `main` |
| `lista` | El nombre que una función le da a la lista que recibe | Solo en esa función |
| `tarea` | Una tarea suelta del `for` | Solo dentro del `for` |

**Cosas nuevas aprendidas:**
- No existe `contador++`. Se escribe `contador += 1;` (la variable tiene que ser `mut`).
- `if` sin paréntesis y con llaves obligatorias; la condición tiene que ser un `bool`:
  ```rust
  if es_urgente(tarea.dias_restantes) {
      contador += 1;
  }
  ```
- Las llaves `{ }` extra dentro de `println!` (por ejemplo `{tarea.titulo}`) forman un bloque que **mueve** el valor y da error. Se pasa el valor sin llaves.
- `.clone()` que sugiere el compilador suele ser un parche. Solo se usa si de verdad hace falta una copia independiente.
- Cada función hace una sola cosa: `es_urgente` solo devuelve `bool`, `mostrar` solo imprime, `contar_urgentes` solo cuenta.

```rust
fn contar_urgentes(lista: &Vec<Tarea>) -> i32 {
    let mut contador: i32 = 0;

    for tarea in lista {
        if es_urgente(tarea.dias_restantes) {
            contador += 1;
        }
    }

    contador            // última línea, sin ;, es lo que devuelve
}
```

Si se olvida esa última línea: `mismatched types: expected i32, found ()`.

## Errores que he visto y qué significan

| Error | Causa |
|---|---|
| No encuentra `Tarea` | El struct estaba dentro de `main` (ámbito) |
| `mostrar()` falla | Se llamó sin pasarle la lista |
| `borrow of moved value` | Usé una variable después de moverla (por ejemplo tras un `push`) |
| Falta un campo al crear `Tarea` | Añadí un campo al struct y no lo puse en cada tarea |
| `cannot borrow as mutable` | La variable dueña no es `mut` |
| `cannot borrow as mutable because it is also borrowed as immutable` | Hay un préstamo de lectura abierto mientras intento modificar |
| `cannot move out of ... behind a shared reference` | Llaves `{ }` extra en `println!` que mueven el valor de un préstamo |
| `mismatched types: expected i32, found ()` | La función no devuelve nada: falta la última línea sin `;` |

## Próximos pasos

Hecho: Fase 0, Fase 1 y el refuerzo de ownership.

Pendiente, en este orden:

1. **Fase 2 (la siguiente): guardar y cargar tareas en un archivo JSON.**
   - `Result` y `Option` (manejo de errores sin excepciones).
   - Crates `serde` y `serde_json` con `cargo add`.
   - `#[derive(Serialize, Deserialize)]` en el struct para convertirlo a JSON.
   - Leer y escribir archivos (`std::fs`) y el operador `?`.
   - Meta: que las tareas no desaparezcan al cerrar el programa.
2. **Fase 3:** proyecto Tauri con la interfaz (el HTML del widget actual).
3. **Fase 4:** conectar la interfaz con comandos de Rust (`#[tauri::command]`: añadir, borrar, completar), con estado compartido (`Mutex`, `State`).
4. **Fase 5:** igualar el widget actual (autoarranque con Windows, estilo, bandeja del sistema).

Ideas futuras: unir las tareas con el calendario (huecos libres, duración de cada tarea) y, más adelante, un servidor propio en casa con IA local.
