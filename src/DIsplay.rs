use std::fmt;//se llama al fmt
struct Structure(i32); //Definimos como se implementara la estructura(tupla)
impl fmt::Display for Structure
{
 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     write!(f,"{}",self.0)
 }
}