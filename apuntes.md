# Apuntes de Rust (proyecto tareas)

Proyecto: gestor de tareas en Rust, con idea de pasarlo luego a Tauri y, a largo plazo, un asistente personal local.
Ruta del proyecto: `D:\dev\personales\sorel` (repo git; el paquete Cargo se llama `task`)

## Dónde me quedé

- Fase 0: terminada.
- Fase 1: terminada (struct, Vec, ownership y préstamo con `&`).
- Refuerzo de ownership (`&mut`, conflicto de préstamos, mover vs prestar, `contar_urgentes`): terminado.
- **Fase 2 (guardar y cargar tareas en JSON): EN CURSO.**
  - Paso 1, `Option` (`buscar_por_titulo`): terminado y correcto.
  - Paso 2, `Result` con `.parse()`: **por aquí me quedé** (ver "Punto exacto" abajo).
  - Pasos 3 y 4 (serde/JSON, `std::fs` y `?`, integrarlo en `main`): pendientes.

### Punto exacto donde lo dejé (para retomar en otro ordenador)

Estoy en `main.rs`, al final de `main`, con el ejercicio de convertir `"3"` (texto) a `i32` con `.parse()` y tratar los dos casos con `match`. El código está **a medias y no compila**:

- El `match texto.parse() { ... }` tiene la rama `Ok()` sin terminar: le falta el valor que recibe (como hacía `Some(i)`) y qué imprimir.
- Falta decirle a `.parse()` a qué tipo convertir (dos formas: anotar el tipo en el `let` o escribir `::<i32>` en la llamada).
- `let mut texto` no necesita `mut`.
- Hay una función vacía `fn texto_parseado (texto) {}` que sobra (sin tipos en el parámetro, no compila). Borrarla o convertirla en una función de verdad con su tipo y su retorno.
- **Limpiar `prse`**: se coló una dependencia equivocada (`prse = "1.2.1"` en `Cargo.toml` y `use prse::parse;`). No se necesita: `.parse()` es un método de `&str` que viene de serie. Quitarla del `Cargo.toml` y del código.

Por hacer en este paso: probar con `"3"` y luego con `"hola"`, imprimiendo el error con `{:?}`, y apuntar qué contiene el `Err`.

Para retomar: `git pull` (o copiar la carpeta), abrir la carpeta que contiene `Cargo.toml` en VS Code y `cargo run`.

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

## Fase 2: Option y Result

**Idea.** Rust no tiene `null` ni excepciones: si algo puede fallar, el fallo va en el tipo que devuelve la función. Son cajas que obligan a tratar todos los casos con `match`.

| Tipo | Significa | Variantes |
|---|---|---|
| `Option<T>` | Hay valor o no hay | `Some(valor)` / `None` (caja vacía, sin explicación) |
| `Result<T, E>` | Salió bien o mal, y por qué | `Ok(valor)` / `Err(error)` (el error trae información) |

- Un `Option` evita inventarse un número para "no encontrado" (el 0 y el -1 se confunden con posiciones reales).
- `usize` es el tipo de las posiciones de una lista.
- Si en un `match` falta un caso, no compila: el error dice exactamente qué falta.

**`Option` en la práctica (hecho):**

```rust
fn buscar_por_titulo(lista: &Vec<Tarea>, titulo: &str) -> Option<usize> {
    for (i, tarea) in lista.iter().enumerate() {
        if tarea.titulo == titulo {
            return Some(i);
        }
    }
    None
}
```

- `.iter().enumerate()` da la posición y el elemento a la vez, en una tupla `(i, tarea)`.
- `return` sale de la función antes de acabar el bucle.
- El `None` final solo se alcanza si el bucle nunca encontró nada.

**`Result` con `.parse()` (en curso):**
- `.parse()` devuelve `Result` porque convertir texto a número puede fallar.
- Es un método (con punto, sin `!`) que ya tienen todos los `&str`: no se importa nada.
- No sabe a qué tipo convertir si no se lo indicas.
- Para depurar un valor que no sabe mostrarse solo se usa `{:?}` en vez de `{}`.

**Método de trabajo que funciona:** hacer cada ejercicio por pasos pequeños y compilar tras cada uno (firma vacía, recorrer, añadir posición, comparar, devolver, `match`).

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
| Sugerencia de import `use prse::parse;` | El editor ofreció un import automático de una librería externa. `parse!(...)` con `!` no es lo mismo que `.parse()` (método de `&str`). No importar nada |

## Próximos pasos

Hecho: Fase 0, Fase 1, el refuerzo de ownership y el paso 1 de la Fase 2 (`Option`).

Pendiente, en este orden:

1. **Fase 2 (en curso): guardar y cargar tareas en un archivo JSON.**
   - `Option` (hecho) y `Result` (en curso): manejo de errores sin excepciones.
   - Crates `serde` y `serde_json` con `cargo add`.
   - `#[derive(Serialize, Deserialize)]` en el struct para convertirlo a JSON.
   - Leer y escribir archivos (`std::fs`) y el operador `?`.
   - Meta: que las tareas no desaparezcan al cerrar el programa.
2. **Fase 3:** proyecto Tauri con la interfaz (el HTML del widget actual).
3. **Fase 4:** conectar la interfaz con comandos de Rust (`#[tauri::command]`: añadir, borrar, completar), con estado compartido (`Mutex`, `State`).
4. **Fase 5:** igualar el widget actual (autoarranque con Windows, estilo, bandeja del sistema).

Ideas futuras: unir las tareas con el calendario (huecos libres, duración de cada tarea) y, más adelante, un servidor propio en casa con IA local.
