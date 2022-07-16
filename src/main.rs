use std::io;
mod game_1431240; //Coloring Game for Adults
mod game_1507410; //Coloring Game for Kids
mod game_914010; //Train Station Renovation

fn main() {
    loop {
        println!("Enter command: "); println!();                                       
        let mut cli_command = String::new();
        io::stdin()
            .read_line(&mut cli_command)
            .expect("Failed to read input");
            match Some(&*cli_command.to_string().trim()) {
                Some("check-progress 1431240") => println!("Checking progress in `Coloring Book for Adults`: {:?}", game_1431240::check_progress()),
                Some("check-progress 1507410") => println!("Checking progress in `Coloring Book for Kids`: {:?}", game_1507410::check_progress()),
                Some("test") => println!("{:?}", game_914010::get_save_dir()),
                Some("quit") => std::process::exit(exitcode::OK),
                _ => println!("Unknown command. Please see instructions on GitHub."),                                             
        }    
    }                                           
}