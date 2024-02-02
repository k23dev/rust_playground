use std::io;

use random_number::random;

fn main() {
    
    let mut input_name:String=String::new();

    let mut play_again:bool=true;
    
    app_banner();
    
    println!("Por favor ingresa tu nombre para jugar:");
    
    io::stdin()
        .read_line(&mut input_name)
        .expect("Error al intentar leer el nombre del usuario.");
    
    let username=input_name.replace("\n", "");
        
    while play_again{
        println!("{username}, he pensado un número entre 1 y 100, y tienes solo ocho intentos.");
        game(&username);
        println!("");
        println!("¿Quieres jugar de nuevo? [S/N]");
        
        let mut option:String=String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Error al leer la opción elegida.");

        let option=option.replace("\n", "");
        let option=option.to_lowercase();
        
        if option=="n" {
            play_again=false;
        }
    }

}

fn app_banner(){
    println!("----------------------------------------");
    println!("    DESAFIO 4: Adivina el número");
    println!("---------------------------------------");
    println!("");

}

fn game(username:&String){

    let mut user_trys:u8=1;

    let max_trys:u8=9;
    
    let mut game_over:bool=false;
    
    let secret_num:u8=generate_secret_num();

    while !game_over{
        
        let mut input_num:String=String::new();

        io::stdin()
            .read_line(&mut input_num)
            .expect("Error al leer el número del jugador");

        let input_num=input_num.replace("\n", "");

        // conver from string to number
        let input_num:u8=input_num.parse().unwrap();
        
        let is_user_winner=check_is_computers_num(secret_num,input_num);

        if is_user_winner{
            println!("¡Felicitaciones {username}! Ganaste, acertaste el número en {user_trys} intentos.");
        }else{
            user_trys+=1;
        }

        if user_trys==max_trys{
            game_over=true;
            println!("Perdiste, el número secreto era: {secret_num}, \n No te desanimes y vuelve a intentarlo.");
        }

    }
}

fn generate_secret_num() -> u8{
    
    let mut num:u8=random!();
    if num > 100{
        num-=100;
    }
    return num;
}

fn check_is_computers_num(secret_num:u8,num :u8) -> bool{

    if secret_num == num{
        return true;
    }else if num < secret_num{
        println!("Respuesta es incorrecta y que ha elegido un número MENOR al número secreto.");
        return false;
    }else if num > secret_num{
        println!("Respuesta es incorrecta y que ha elegido un número MAYOR al número secreto.");
        return false;
    }else{
        println!("El número elegido debe ser entre 1 y 100");
        return false;
    }


}