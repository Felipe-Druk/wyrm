//! Catálogo de ejemplos precargados escritos en sintaxis válida de Wyrm.

/// Representa un programa de ejemplo que se puede elegir en el Playground.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub code: &'static str,
    pub default_stdin: &'static str,
}

/// Lista de ejemplos incluidos en el selector superior.
pub const EXAMPLES: &[Example] = &[
    Example {
        id: "aritmetica",
        title: "1. Aritmética y variables",
        description: "Tipado explícito (int32, int64, nat8, bool), operadores y retorno implícito de bloques.",
        code: r#"// Ejemplo 1: Aritmética, tipos explícitos y bloques con retorno implícito
int32 base = 6
int32 altura = 7
int64 area_doble = (base * altura) + 2 ^ 3

// En Wyrm los caracteres son bytes nativos (nat8)
nat8 inicial = 'W'
bool distinto = (area_doble ~= 0) and (inicial == 'W')

// Todo bloque { ... } evalúa y retorna su última expresión
int64 resultado = 0
if (distinto) {
    resultado = {
        int64 ajuste = 10
        area_doble + ajuste
    }
}

write(1, "Valor evaluado en Wyrm!\n")
resultado // Retorno implícito: 60"#,
        default_stdin: "",
    },
    Example {
        id: "control_flujo",
        title: "2. Control de flujo y bucles",
        description: "Sentencias if / else if / else, desigualdad ~= y bucle while.",
        code: r#"// Ejemplo 2: Control de flujo (if/else if/else) y bucles while
int32 tope = 12
int32 i = 1
int32 suma_filtrada = 0

while (i <= tope) {
    // Descarta múltiplos de 4 usando el operador de desigualdad ~=
    if (i % 4 ~= 0) {
        if (i % 2 == 0) {
            suma_filtrada = suma_filtrada + (i * 2)
        } else if (i > 5) {
            suma_filtrada = suma_filtrada + i
        } else {
            suma_filtrada = suma_filtrada + 1
        }
    }
    i = i + 1
}

write(1, "Bucle while finalizado.\n")
suma_filtrada // Retorno implícito: 66"#,
        default_stdin: "",
    },
    Example {
        id: "vectores_for",
        title: "3. Vectores y for",
        description: "Vectores dinámicos vec<T>, cadenas vec<nat8> cerradas con 0 y bucle for (n <- vec).",
        code: r#"// Ejemplo 3: Vectores dinámicos vec<T>, cadenas vec<nat8> y bucle for
vec<int32> puntajes = [12, 25, 38, 15, 10]
puntajes[0] = 20 // Los vectores son mutables por índice

int32 total = 0
for (n <- puntajes) {
    if (n ~= 10) {
        total = total + n
    }
}

// Las cadenas son vec<nat8> cerradas con byte nulo (0)
vec<nat8> saludo = "Hola desde Wyrm!\n"
nat16 largo = 0
while (saludo[largo] ~= 0) {
    largo = largo + 1
}

write(1, saludo)
total // 20 + 25 + 38 + 15 = 98"#,
        default_stdin: "",
    },
    Example {
        id: "funciones",
        title: "4. Funciones de primera clase",
        description: "Declaración de funciones y lambdas con firma explícita (Tipos) -> Retorno.",
        code: r#"// Ejemplo 4: Funciones y lambdas con firma explícita (Tipos) -> Retorno
(vec<nat8>) -> int64 imprimir = (texto) -> {
    write(1, texto)
}

(int32) -> int32 cuadrado = (x) -> {
    x * x
}

(int32, int32) -> int32 suma_de_cuadrados = (a, b) -> {
    int32 ca = cuadrado(a)
    int32 cb = cuadrado(b)
    ca + cb
}

(nat64) -> nat64 factorial = (n) -> {
    nat64 acc = 1
    nat64 k = 2
    while (k <= n) {
        acc = acc * k
        k = k + 1
    }
    acc
}

imprimir("Evaluando funciones en Wyrm...\n")
int64 suma_cuadrados = suma_de_cuadrados(3, 4)
nat64 fact5 = factorial(5)

// Bloque final con retorno implícito
{
    int64 total_final = suma_cuadrados + fact5
    total_final // 25 + 120 = 145
}"#,
        default_stdin: "",
    },
];

/// Devuelve el primer ejemplo como programa inicial del Playground.
pub fn default_example() -> &'static Example {
    &EXAMPLES[0]
}

/// Busca un ejemplo por su identificador estable.
pub fn find_by_id(id: &str) -> Option<&'static Example> {
    EXAMPLES.iter().find(|example| example.id == id)
}
