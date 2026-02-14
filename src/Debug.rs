#[derive(Debug)]
struct Structure(i32);
#[derive(Debug)]
struct Deep(Structure);
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}
fn main() {
// Imprimir con `{:?}` es similar a imprimir con `{}`.
println!("{:?} meses en un año.", 12);

    /*
    println!("{1:?} {0:?} es el nombre del {actor:?}.",
         0---> "Slater",
         1---> "Christian",
         actor="actor's");
     */
println!("{1:?} {0:?} es el nombre del {actor:?}.",
         "Slater",
         "Christian",
         actor="actor's");

// ¡`Structure` también se puede imprimir!
println!("Ahora {:?} imprimirá ", Structure(3));

// El problema con 'derive` es que no hay control sobre cómo
// los resultados se ven. ¿Qué pasa si quiero que esto solo muestre un `7`?
println!("Ahora {:?} imprimirá ", Deep(Structure(7)));
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
    // // Impresión bonita

}