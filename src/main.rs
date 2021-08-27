use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("expected command");

    if command == "save" {
        //command is in form: secrets-storage save <project> <file>

        let project = args.next().expect("no project provided");
        let file_str = args.next().expect("no file provided");

        let path_str = format!("C:/Users/matth/secrets-storage/projects/{}/", project);
        let path = Path::new(&path_str);
        std::fs::create_dir_all(path).expect("failed to create path to store files");

        let file = Path::new(&file_str).file_name().expect("failed to retrieve file name");
        let full_path = path.join(file);
        std::fs::copy(file_str, full_path).expect("failed to save file");
    } else if command == "get" {
        //command is in form: secrets-storage get <project> <file> <location>
        let project = args.next().expect("no project provided");
        let file = args.next().expect("no file provided");
        let mut location = args.next().expect("no location provided");
        location.push_str(&file);
        let path = format!("./projects/{}/{}", project, file);
        std::fs::copy(path, location).expect("failed to retrieve file");
    }
}
