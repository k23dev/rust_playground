use std::io;

fn main() {

    let mut input_text:String=String::new();
    let mut input_letters:Vec<char>=vec![];

    app_banner();

    println!("");
    println!("Escribe un texto para trabajar por favor:\n");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Error al intentar leer el input");

    
    let input_text=input_text.replace("\n", "");
    let input_text:String=String::from(input_text.trim());

    println!("Ahora escribe 3 letras para buscar dentro del texto anterior:");
    for _n in 1..4{
        let mut input_letter_aux:String=String::new();
        io::stdin()
            .read_line(&mut input_letter_aux)
            .expect("Error al intentar leer la vocal");

        let input_letter_aux=input_letter_aux.replace("\n", "");

        let letter=input_letter_aux.chars().nth(0).unwrap();
        input_letters.push(letter);
   }

    count_letters_in_text(&input_text, &input_letters);
    count_words_in_text(&input_text);
    first_and_last_in_text(&input_text);
    reverse_text(&input_text);
    is_rust_in_text(&input_text);

}

fn app_banner(){
    println!("----------------------------------------");
    println!("    DESAFIO 3: Análisis de textos");
    println!("---------------------------------------");
    println!("");

}

fn app_separator(){
    println!("");
    println!("+++");
    println!("");

}

fn count_letters_in_text(text :&String, letters:&Vec<char>){
    app_separator();
   
   let text_aux=text.clone();
//    let lc=text_aux.chars();

   let mut counter:Vec<u16>=vec![0,0,0];

   for l in text_aux.chars(){

        for n in 0..3{
            if l==letters[n]{
                counter[n]=counter[n]+1;
            }
        }
   }

    for n in 0..3{
        println!("La cantiad de veces que aparece \"{}\" son {} veces",letters[n],counter[n]);
    }

}

fn count_words_in_text(text :&String){

    app_separator();

    let text_aux=text.clone();

    let wc=text_aux.split(" ").count();

    println!("La cantidad de palabras del texto es: {wc}");



}

fn first_and_last_in_text(text :&String){
    app_separator();

    let first_letter=&text[0..1];
    let last_letter=text.chars().last().unwrap();

    println!("La primer letra del texto es \"{first_letter}\" y la última es \"{last_letter}\"");


}

fn reverse_text(text :&String){
    app_separator();


    let text_rev=text.clone();
    let text_rev=text_rev.chars().rev().collect::<String>();

    println!("{text_rev}");

}

fn is_rust_in_text(text :&String){
    
    let text_aux=text.clone();
    
    let text_aux=text_aux.to_lowercase();
    app_separator();
    print!("La palabra \"rust\" ");
    if !text_aux.contains("rust"){
        print!("NO ");
    }
    print!("existe en el texto.\n")
}