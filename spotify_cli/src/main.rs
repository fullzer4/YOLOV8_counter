use std::env;

mod auth {
  pub mod login;
}

use auth::login::Login;

fn main() {

  let args: Vec<String> = env::args().collect();

   if args.len() > 1 {
      for arg in args.iter().skip(1) {
        
        if arg == "login" {

          Login()
        
        } else if arg == "play" {
        
          println!("login");
        
        } 

      }
    }
}