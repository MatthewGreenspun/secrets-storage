use std::{io::Write, path::Path};

fn main(){
    let mut args = std::env::args().skip(1);
    let command = match args.next(){
        Some(command) => command,
        None => {
            println!("\ncommands:\n\n\tsecrets-storage save <project> <file>\n\tsecrets-storage get <project> <file> <location - optional>");
            return;
        }
    };

    if command == "save" {
        let project = match args.next(){
            Some(project) => project,
            None => {
                println!("error: no project provided\nfull command is: \n\tsecrets-storage save <project> <file>");
                return;
            },
        };
        let file_str = match args.next(){
            Some(file_str) => file_str,
            None => {
                println!("error: no file provided\nfull command is: \n\tsecrets-storage save <project> <file>");
                return;
            },
        };
        match save_secrets(project, file_str) {
            Ok(_) => println!("file saved successfully"),
            Err(_) => println!("error saving file"), 
        }

    } else if command == "get" {
        let project = match args.next(){
            Some(project) => project,
            None => {
                println!("error: no project provided\nfull command is: \n\tsecrets-storage get <project> <file> <location - optional>");
                return;
            },
        };
        let file_str = match args.next(){
            Some(file_str) => file_str,
            None => {
                println!("error: no file provided\nfull command is: \n\tsecrets-storage get <project> <file> <location - optional>");
                return;
            },
        };
        let location_str = match args.next(){
            Some(location) => location,
            None => String::from("./"),
        };

        match get_secrets(project, file_str, location_str) {
            Ok(message) => println!("{}", message),
            Err(_) => println!("error retrieving file"), 
        }
    } else {
        println!("{} is not a valid command", command);
    }
}

fn save_secrets(project: String, file_str: String) -> std::io::Result<()>{
    //command is in form: secrets-storage save <project> <file>

    let storage_path_str = format!("C:/Users/matth/secrets-storage/projects/{}/", project);
    let storage_path = Path::new(&storage_path_str);
    if !storage_path.exists(){
        std::fs::create_dir_all(storage_path)?;
    }

    let file = Path::new(&file_str).file_name().expect("failed to retrieve file name");
    let full_path = storage_path.join(file);
    std::fs::copy(file_str, full_path)?;
    Ok(())
}

fn get_secrets(project: String, file_str: String, location_str: String) -> std::io::Result<&'static str>{
    //command is in form: secrets-storage get <project> <file> <location>
    //moves stored file to location

    let storage_path_str = format!("C:/Users/matth/secrets-storage/projects/{}/{}", project, file_str);

    let file = Path::new(&file_str);
    let location = Path::new(&location_str);
    let full_location = location.join(file);
    
    let mut input = String::new();
    if full_location.exists() {
        print!("file {} already exists. Would you like to replace it?[y/n] ", &full_location.to_str().unwrap());
        let _  = std::io::stdout().flush();
        std::io::stdin().read_line(&mut input).expect("failed to read user input");
    }

    if input.trim() == "y" || !full_location.exists() {
        std::fs::copy(storage_path_str, full_location)?;
        return Ok("retrieved file successfully");
    }
    Ok("")
}