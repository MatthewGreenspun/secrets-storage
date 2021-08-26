fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("expected command");

    if command == "save" {
        //command is in form: secrets-storage save <project> <file>
        let project = args.next().expect("no project provided");
        let file = args.next().expect("no file provided");
        let path = format!("./projects/{}/{}", project, file);
        std::fs::create_dir(format!("./projects/{}", project)).expect("failed to create path to store files");
        std::fs::copy(file, path.clone()).expect("failed to save file");
    } else if command == "get" {
        //command is in form: secrets-storage get <project> <file> <location>
        let project = args.next().expect("no project provided");
        let file = args.next().expect("no file provided");
        let location = args.next().expect("no location provided");
        let path = format!("./projects/{}/{}", project, file);
        std::fs::copy(path,location).expect("failed to retrieve file");
    }
}
