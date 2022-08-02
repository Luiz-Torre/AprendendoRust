const PI:f32 = 3.14;
static GLOBAL:u8 = 1;
fn main() {


    println!("PI = {}", PI);
    println!("Variavel Global = {}", GLOBAL);

    let variavel:i32 = 128;
    println!("variavel = {} , tam = {}", variavel, std::mem::size_of_val(&variavel));

    let decimal = 2.5;
    println!("decimal = {}", decimal);

    let boolean = false;
    println!("boolean : {}, tam = {}", boolean, std::mem::size_of_val(&boolean));

    let letra:char = 'a';
    println!("letra = {}, tam = {}", letra,std::mem::size_of_val(&letra));
}
