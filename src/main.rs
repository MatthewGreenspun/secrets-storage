use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("expected command");

    if command == "save" {
        //command is in form: secrets-storage save <project> <file>

        let project = args.next().expect("no project provided");
        let file_str = args.next().expect("no file provided");

        let storage_path_str = format!("C:/Users/matth/secrets-storage/projects/{}/", project);
        let storage_path = Path::new(&storage_path_str);
        if !storage_path.exists(){
            std::fs::create_dir_all(storage_path).expect("failed to create path to store files");
        }

        let file = Path::new(&file_str).file_name().expect("failed to retrieve file name");
        let full_path = storage_path.join(file);
        std::fs::copy(file_str, full_path).expect("failed to save file");
    } else if command == "get" {
        //command is in form: secrets-storage get <project> <file> <location>
        //moves stored file to location

        let project = args.next().expect("no project provided");
        let file_str = args.next().expect("no file provided");
        let location_str = args.next().expect("no location provided");
        let storage_path_str = format!("C:/Users/matth/secrets-storage/projects/{}/{}", project, file_str);

        let file = Path::new(&file_str);
        let location = Path::new(&location_str);
        let full_location = location.join(file);

        std::fs::copy(storage_path_str, full_location).expect("failed to retrieve file");
    }
}
