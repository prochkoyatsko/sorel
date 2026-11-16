# Apuntes de Rust (proyecto tareas)

Proyecto: gestor de tareas en Rust, con idea de pasarlo luego a Tauri y, a largo plazo, un asistente personal local.
Ruta del proyecto: `D:\proyectos\personales\sorel` (repo git; el paquete Cargo se llama `task`)

## Dónde me quedé

- Fase 0: terminada.
- Fase 1: terminada (struct, Vec, ownership y préstamo con `&`).
- Refuerzo de ownership (`&mut`, conflicto de préstamos, mover vs prestar, `contar_urgentes`): terminado.
- **Fase 2 (guardar y cargar tareas en JSON): TERMINADA.** `Option`, `Result`, `?`, `serde`/`serde_json` y `std::fs`. Las tareas sobreviven al cerrar el programa.
- Mejora de la Fase 2 ya hecha: si `tareas.json` no existe, `cargar` devuelve una lista vacía (probado: sin archivo, con archivo y con archivo roto).
- **Siguiente: pequeña limpieza de `main` y luego Fase 3 (Tauri).**

### Punto exacto donde lo dejé (para retomar en otro ordenador)

`main.rs` tiene `cargar()` y `guardar(&lista)` funcionando, y `cargar` ya maneja el caso de que no exista el archivo. `main` carga `tareas.json`, añade una tarea con `push` (solo es una prueba: **cada `cargo run` añade otra**), guarda e imprime el total.

**Antes de seguir, revisar al retomar:**
- En la carpeta `sorel` hay un `tareas.json` (creado al probar la lista vacía, con 1 sola tarea) y un `tareas_old.json` (el original, con las tareas de antes). Para recuperarlo: borrar `tareas.json` y renombrar `tareas_old.json` a `tareas.json`. Si no se quieren los datos viejos, borrar `tareas_old.json`.
- Decidir si `tareas.json` va al repo o se añade a `.gitignore` (son datos personales). Al terminar de probar, borrar `tareas_old.json`.
- Hay cambios sin commit (`Cargo.toml`, `Cargo.lock`, `main.rs`, `apuntes.md`). Hacer el commit `day-3` o `day-4`.

Pequeña limpieza pendiente en `main`:
- Quitar el `push` de prueba (si no, el archivo crece con cada `cargo run`).
- Recuperar `buscar_por_titulo` (está comentada) y usarla sobre la lista cargada.
- Borrar los bloques de código comentado antiguo.
- Pasar `cargo fmt`.

Para retomar: `git pull` (o copiar la carpeta), abrir la carpeta que contiene `Cargo.toml` en VS Code y `cargo run` (desde esa carpeta, porque `tareas.json` se busca ahí).

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

**`Result` con `.parse()` (hecho).** Analogía: `.parse()` es un repartidor que trae un **paquete**. Dentro hay `Ok(valor)` (llegó bien) o `Err(motivo)` (no se pudo entregar, con una nota).

```rust
match texto.parse::<i32>() {
    Ok(num) => println!("OK: {}", num),
    Err(err) => println!("Error: {:?}", err),
}
```

- `.parse()` devuelve `Result` porque convertir texto a número puede fallar.
- Es un método (con punto, sin `!`) que ya tienen todos los `&str`: no se importa nada.
- No sabe a qué tipo convertir si no se lo indicas: `::<i32>` en la llamada o el tipo en el `let`.
- Con `"hola"`, `{:?}` muestra `ParseIntError { kind: InvalidDigit }` y `{}` el mensaje `invalid digit found in string`.
- `{:?}` sirve para depurar un valor que no sabe mostrarse con `{}`.

**Una función que devuelve `Result`** pasa el paquete cerrado a quien la llama (el "almacén"):

```rust
fn convertir(texto: &str) -> Result<i32, std::num::ParseIntError> {
    texto.parse::<i32>()          // sin ; ni match: devuelve el paquete tal cual
}
```

**El operador `?`** abre el paquete dentro de la función: si es `Ok` da el valor limpio, si es `Err` sale de la función ya y devuelve ese error.

```rust
fn convertir_sumar(texto: &str) -> Result<i32, std::num::ParseIntError> {
    let num = convertir(texto)?;   // num es un i32 normal
    Ok(num + 10)                   // se vuelve a cerrar el paquete
}
```

- Un `Result` no se puede sumar: `num += 10` sobre un `Result` da `cannot be applied to type Result`. Primero hay que abrirlo con `?` o con `match`.
- Una operación (`+=`) devuelve `()`, no un `Result`: si la función promete `Result`, la última línea debe ser `Ok(...)`.
- Pedir el permiso mínimo: si no se cambia la variable, sin `mut`.
- `?` solo se puede usar en una función que devuelve `Result` (o `Option`).

**Método de trabajo que funciona:** hacer cada ejercicio por pasos pequeños y compilar tras cada uno (firma vacía, recorrer, añadir posición, comparar, devolver, `match`).

## Fase 2 (continuación): serde, JSON y archivos

**Para qué sirve `serde`.** El `Vec<Tarea>` vive en la RAM y desaparece al cerrar el programa. Hay que convertirlo a texto y guardarlo en un archivo (analogía: una mudanza, se desmonta la casa en cajas y se guarda en un trastero).

| Dirección | Nombre | Equivalente en JS |
|---|---|---|
| `Tarea` a texto JSON | serializar (`Serialize`) | `JSON.stringify` |
| texto JSON a `Tarea` | deserializar (`Deserialize`) | `JSON.parse` |

- `serde` define las reglas generales y `serde_json` las aplica al formato JSON. Otros formatos (`toml`, `yaml`) usarían los mismos structs.
- Instalar: `cargo add serde --features derive` y `cargo add serde_json`. Sin `features = ["derive"]` no existe la etiqueta `#[derive(...)]`.
- Etiqueta en el struct, sin escribir nada más:

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Tarea { titulo: String, completada: bool, dias_restantes: i32 }
```

**Convertir la lista entera, no tarea por tarea con un `for`.** `Vec<Tarea>` también sabe convertirse solo. Con un `for` saldrían varios JSON sueltos (no válido), con el `Vec` sale un array `[{...},{...}]`.

- `serde_json::to_string(&lista)`: JSON en una línea.
- `serde_json::to_string_pretty(&lista)`: con saltos de línea y sangría (más cómodo de leer y en git).
- `serde_json::from_str::<Vec<Tarea>>(&texto)`: del texto a la lista. El `::<Vec<Tarea>>` es como el `::<i32>` de `.parse()`: el JSON no dice a qué tipo montarlo.
- Las dos pueden fallar, así que devuelven `Result`.

**Archivos con `std::fs`:**

- `std::fs::write("tareas.json", json)`: **sobrescribe todo el contenido** cada vez (no añade al final). Devuelve `Result<(), std::io::Error>`.
- `std::fs::read_to_string("tareas.json")`: devuelve `Result<String, std::io::Error>`.
- `()` (vacío) en `Result<(), ...>` significa "si sale bien no hay dato que devolver".
- La ruta relativa se busca desde la carpeta **desde la que se ejecuta `cargo run`** (la que tiene `Cargo.toml`), no desde donde está el `.rs`. Con `"../tareas.json"` no la encuentra.
- Riesgo de sobrescribir: guardar una lista vacía o rota pisa las tareas reales. Por eso conviene cargar antes de guardar.

**`Box<dyn std::error::Error>`.** `read_to_string` falla con `std::io::Error` y `from_str` con `serde_json::Error`, pero una función solo declara un tipo de error. `Box<dyn Error>` es "una caja que acepta cualquier error", y así `?` funciona con los dos.

```rust
fn cargar() -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    let texto = std::fs::read_to_string("tareas.json")?;
    let lista = serde_json::from_str::<Vec<Tarea>>(&texto)?;
    Ok(lista)
}

fn guardar(lista: &Vec<Tarea>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(lista)?;
    std::fs::write("tareas.json", json)?;
    Ok(())
}
```

- `cargar` devuelve datos (`Result<Vec<Tarea>, ...>`); `guardar` solo hace algo (`Result<(), ...>`).
- `lista` ya es `&Vec<Tarea>`, así que a `to_string_pretty` se le pasa sin otro `&`.
- La lista cargada **tiene que salir** de la función. Si se carga dentro de un `match` anidado, se pierde al cerrar la llave y `buscar_por_titulo` buscaba en una lista vacía.
- Dos `match` anidados (abrir un paquete dentro de otro) son feos: `?` en una función los elimina.

Uso en `main`:

```rust
match cargar() {
    Ok(mut mis_tareas) => {            // mut para poder hacer push
        mis_tareas.push(Tarea { ... });
        match guardar(&mis_tareas) {
            Ok(_) => println!("Guardado"),
            Err(err) => println!("Error: {}", err),
        }
    }
    Err(err) => println!("Error: {}", err),
}
```

- Dentro de las llaves de una rama va código normal. `Ok(mis_tareas) => { Ok(lista) => ... }` mezcla dos `match` y no compila.
- Si no existe el archivo, la rama `Err` imprime `El sistema no puede encontrar el archivo especificado. (os error 2)`. `os error 2` es "archivo no encontrado" en Windows.

**Empezar con una lista vacía (primera vez que se abre la app).** Dos casos distintos que no hay que mezclar:

| Caso | Qué hacer |
|---|---|
| El archivo **no existe** | Lista vacía, es normal |
| El archivo existe pero está **roto** | Error, y no tocar nada |

La trampa es `Err(_) => Vec::new()`: se traga todos los errores, y con un archivo corrupto se empezaría en vacío y al guardar se **pisarían las tareas reales**. Por eso la comprobación va **dentro de `cargar`**, antes de leer:

```rust
fn cargar() -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    if !std::path::Path::new("tareas.json").exists() {
        return Ok(Vec::new());          // lista vacía, envuelta en Ok
    }
    let texto = std::fs::read_to_string("tareas.json")?;
    let lista = serde_json::from_str::<Vec<Tarea>>(&texto)?;
    Ok(lista)
}
```

- `Path::new("...").exists()` devuelve un `bool`; `!` delante significa "no".
- `return` sale de la función antes de acabar y lleva `;` (es una instrucción).
- No hace falta anotar el tipo de `Vec::new()`: la firma de `cargar` ya dice que es `Vec<Tarea>`.
- **El `Ok(...)` tiene que coincidir con la firma.** Poner ese `if` en `guardar` (que promete `Result<(), ...>`) da `expected (), found Vec<_>`. Además, en `guardar` habría impedido guardar la primera vez.

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
| `binary assignment operation += cannot be applied to type Result` | Intenté sumar a un `Result` sin abrirlo. Hay que abrirlo antes con `?` o `match` |
| `expected Result, found ()` | La última línea de la función es una operación (`+=`) y no un `Ok(...)` |
| `expected one of ... found =>` | Puse `Ok(x) => ...` dentro de las llaves de otra rama en vez de código normal |
| `unused variable: json` | Hice la conversión dos veces: una guardada en una variable sin usar y otra en el `match`. Dejar solo una |
| `os error 2` (el sistema no encuentra el archivo) | La ruta del archivo está mal (`../tareas.json`, `tareas2.json`) o el archivo no existe |
| `mismatched types: expected (), found Vec<_>` | Puse `return Ok(Vec::new())` en una función que promete `Result<(), ...>` (`guardar`). El `Ok(...)` tiene que coincidir con la firma de esa función |

## Próximos pasos

Hecho: Fase 0, Fase 1, el refuerzo de ownership y la Fase 2 completa (`Option`, `Result`, `?`, `serde`, `serde_json`, `std::fs`).

Pendiente, en este orden:

1. **Limpieza final de la Fase 2:** quitar el `push` de prueba de `main`, recuperar `buscar_por_titulo` sobre la lista cargada, borrar el código comentado y pasar `cargo fmt`. (La lista vacía cuando no existe el archivo ya está hecha.)
2. **Fase 3:** proyecto Tauri con la interfaz (el HTML del widget actual).
3. **Fase 4:** conectar la interfaz con comandos de Rust (`#[tauri::command]`: añadir, borrar, completar), con estado compartido (`Mutex`, `State`).
4. **Fase 5:** igualar el widget actual (autoarranque con Windows, estilo, bandeja del sistema).

Ideas futuras: unir las tareas con el calendario (huecos libres, duración de cada tarea) y, más adelante, un servidor propio en casa con IA local.
