use std::io;


fn main() {
    app_banner();

    println!("Nombre del vendedor/ora:");

    let mut nombre:String=String::new();
    
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer el nombre del vendedor/ora");


    let nombre=nombre.replace("\n", "");
    println!("");
    println!("Por favor ingresa tus ventas. Preciona 0 para salir y calcular tu porcentaje de ventas.");

    let mut salir:bool=true;
    
    let mut ventas:Vec<f32>=Vec::new();
    
    while salir{

        let mut ingreso:String=String::new();
        
        io::stdin()
            .read_line(&mut ingreso)
            .expect("Error al leer el nuevo ingreso");

        // remove the \n char
        let ingreso=ingreso.replace("\n", "");
        
        if ingreso=="0"{
            salir=false;
        }else{
            let ingreso=ingreso.parse::<f32>().expect("Error al interntar convertir el ingreso a f32");
            ventas.push(ingreso);
        }
    }

    println!("");
    println!("Calculando porcentaje de ventas...");
    println!("");

    let resultado=calcular_comision(&ventas);
    let resultado=format!("{resultado}");
    let resultado=resultado.replace(".", ",");

    println!("{nombre}, el total de tu comisión es de ${resultado}");

}

fn app_banner(){
    println!("----------------------------------------");
    println!("    DESAFIO 2: Calcular comisiones");
    println!("---------------------------------------");
    println!("");

}

fn calcular_comision(ventas :&Vec<f32>) -> f32 {

    let mut result:f32=0.0;

    for venta in ventas{
        result+=venta * 13.00 / 100.00;
    }

    result
}