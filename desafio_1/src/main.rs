use std::io;

fn main() {
    app_banner();

    let mut input_color:String= String::new();
    let mut input_animal:String=String::new();

    println!("Amigo, voy a ayudarte con tu empresa. Le daré un nombre de tu cervecería en base a dos preguntas fundamentales.\n ¿Estás listo? ¡Vamos!");
    println!("");
    println!("¿Cuál es tu color favorito?");
    
    io::stdin()
        .read_line(&mut input_color)
        .expect("Error al intentar leer el color favorito");

   println!("¿Cuál es tu animal favorito?");

   io::stdin()
        .read_line(&mut input_animal)
        .expect("Error al leer el animal favorito");

    println!("");
    println!("Mmm... el nombre de tu cervecería será...");
    println!("Será ...");
    let input_animal=input_animal.replace("\n", "");

    let ultima_letra=input_animal.chars().last().unwrap();

    let mut articulo="El";

    if ultima_letra=='a'{
        articulo="La";
    }

    print!("{articulo} {input_animal} {input_color}");


}

fn app_banner(){

    println!("----------------------------------------");
    println!("  DESAFIO 1: Nombre de cervecería");
    println!("---------------------------------------");
    println!("");

}