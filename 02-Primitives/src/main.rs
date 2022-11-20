fn main() {


    // Tipos especificados
    let logical: bool = true;
    let a_float: f64 = 1.0; // Anotacion regular
    let an_integer = 5i32; // Anocation por sufijo

    // Tipos por defecto 
    let default_float = 3.0; // f64
    let default_integer = 7; // i32
    
    let mut inferred_type = 12; // Tipo i64 inferido de la linea inferior
    inferred_type = 4294967296i64;

    // El valor de la variable puede cambiar
    let mut mutable = 12;
    mutable = 21;
    
    // Error! El tipo de la variable no puede cambiar
    // mutable = true;

    // Las variables se pueden sobreescribir con Shadowing.
    let mutable = true;

}
