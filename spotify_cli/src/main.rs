use std::env;
use std::io;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut email =  String::new();
    let mut password = String::new();

    if args.len() > 1 {
      for arg in args.iter().skip(1) {
        
        if arg == "login" {

            println!("Entre com seu dados para login no spotify: \n");
    
            println!("Email:");
            io::stdin().read_line(&mut email).expect("Falha ao ler entrada");
            
            println!("");

            println!("Password: ");
            io::stdin().read_line(&mut password).expect("Falha ao ler entrada");

            // armazenar como variaveis de ambiente e realizar o login
            


            env::set_var("SPOTIFYCLI_EMAIL", &email);
            env::set_var("SPOTIFYCLI_PASSWORD", &password);

        } else if arg == "play" {
        
            println!("play");
        
            // aplicar o play so se tiver ja logado por verificacao no env
        } 

      }
    }
}
