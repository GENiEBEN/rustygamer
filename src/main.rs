use std::io;
mod game_1431240;
mod game_914010;

fn main() {
    loop {
        println!("Enter command: "); println!();                                       
        let mut cli_command = String::new();
        io::stdin()
            .read_line(&mut cli_command)
            .expect("Failed to read input");
            match Some(&*cli_command.to_string().trim()) {
                Some("check-progress 1431240") => println!("Checking progress in `Coloring Book for Adults`: {:?}", game_1431240::check_progress()),
                Some("test") => println!("{:?}", game_914010::get_save_dir()),
                Some("quit") => std::process::exit(exitcode::OK),
                _ => println!("Unknown command. Please see instructions on GitHub."),                                             
        }    
    }                                           
}