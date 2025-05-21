// Se define listaa con estructura cons utilizando la estructura box para resolver el problema de recursividad en el Stack.
// Se utilizan genericos para adaptar las listas si necesita el ingreso de otro tipo de dato. 
#[derive(Debug)]
enum Lista<T> {
    Cons(T, Box<Lista<T>>), 
    Nil,                    
}

// Listado de piezas de los automóvil
#[derive(Debug)]
struct Pieza(String);

// Listado de modelos de automovil con su respectivo listado de piezas 
#[derive(Debug)]
struct Modelo {
    nombre: String,
    piezas: Lista<Pieza>,
}

// Listado de marcas de automovil con sus respectivos modelos 
#[derive(Debug)]
struct Marca {
    nombre: String,
    modelos: Lista<Modelo>,
}

// Consecionario con su respectivo listado de marcas 
type Concesionario = Lista<Marca>;


// Imprime las listas a base de genericos 
fn imprimir_lista<T, F>(lista: &Lista<T>, imprimir_item: &F)
where
    F: Fn(&T),
{
    match lista {
        Lista::Cons(head, tail) => {
            imprimir_item(head);         
            imprimir_lista(tail, imprimir_item); 
        }
        Lista::Nil => {
        }
    }
}

// Imprime todas las listas acorde a la jerarquia marca - modelo - piezas 
fn imprimir_concesionario(concesionario: &Concesionario) {
    imprimir_lista(concesionario, &|marca| {
        println!("Marca: {}", marca.nombre);
        imprimir_lista(&marca.modelos, &|modelo| {
            println!("  Modelo: {}", modelo.nombre);
            imprimir_lista(&modelo.piezas, &|pieza| {
                println!("    Pieza: {}", pieza.0);
            });
        });
    });
}

fn main() {
    // Se crean las diferentes listas para definir los elementos de la marca Ferrari
    let piezas_f40 = Lista::Cons(
        Pieza("piezaAFF".to_string()),
        Box::new(Lista::Cons(Pieza("piezaBFF".to_string()), Box::new(Lista::Cons(Pieza("piezaCFF".to_string()),
         Box::new(Lista::Nil))))),
    );

    let piezas_330p4 = Lista::Cons(
        Pieza("piezaAFP".to_string()),
        Box::new(Lista::Cons(Pieza("piezaBFP".to_string()), Box::new(Lista::Cons(Pieza("piezaCFP".to_string()),
         Box::new(Lista::Nil))))),
    );

    let modelo_f40 = Modelo {
        nombre: "F40 GT".to_string(),
        piezas: piezas_f40,
    };

    let modelo_330p4 = Modelo {
        nombre: "330 p4".to_string(),
        piezas: piezas_330p4,
    };
 
    let modelos_ferrari = Lista::Cons(modelo_f40, Box::new(Lista::Cons(modelo_330p4, Box::new(Lista::Nil))));

    let ferrari = Marca {
        nombre: "Ferrari".to_string(),
        modelos: modelos_ferrari,
    };

    // Se crean las diferentes listas para definir los elementos de la marca Porshe 
    let piezas_718 = Lista::Cons(
        Pieza("piezaA7P".to_string()),
        Box::new(Lista::Cons(Pieza("piezaB7P".to_string()), Box::new(Lista::Cons(Pieza("piezaC7P".to_string()),
        Box::new(Lista::Nil))))),
    );

    let modelo_718 = Modelo {
        nombre: "718".to_string(),
        piezas: piezas_718,
    };

    let piezas_911 = Lista::Cons(
        Pieza("piezaA9P".to_string()),
        Box::new(Lista::Cons(Pieza("piezaB9P".to_string()), Box::new(Lista::Cons(Pieza("piezaC9P".to_string()),
        Box::new(Lista::Nil))))),
    );

    let modelo_911 = Modelo {
        nombre: "911".to_string(),
        piezas: piezas_911,
    };

    let modelos_porshe = Lista::Cons(modelo_718, Box::new(Lista::Cons(modelo_911, Box::new(Lista::Nil))));

    let porshe = Marca {
        nombre: "Porshe".to_string(),
        modelos: modelos_porshe,
    };

    // Se crean las diferentes listas para definir los elementos de la marca Nissan 
    let piezas_gtr = Lista::Cons(
        Pieza("piezaAGN".to_string()),
        Box::new(Lista::Cons(Pieza("piezaBGN".to_string()), Box::new(Lista::Cons(Pieza("piezaCGN".to_string()),
        Box::new(Lista::Nil))))),
    );

    let modelo_gtr = Modelo {
        nombre: "GTR".to_string(),
        piezas: piezas_gtr,
    };

    let modelos_nissan = Lista::Cons(modelo_gtr, Box::new(Lista::Nil));

    let nissan = Marca {
        nombre: "Nissan".to_string(),
        modelos: modelos_nissan,
    };

    // Definicion final de todos los elementos del concesionario 
    let concesionario = Lista::Cons(ferrari, Box::new(Lista::Cons(porshe, Box::new(Lista::Cons(nissan, Box::new(Lista::Nil))))),
    );

    // Retornar el listado final del consesionario 
    imprimir_concesionario(&concesionario);
}
