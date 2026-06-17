use log_analyzer::args::Args;
fn main() {
    match Args::parse() {
        Ok(args) => {
            println!("Файл: {}", args.file_path.display());
            println!("Паттерн {}", args.pattern);
            if args.quiet {
                println!("Тихий режим включен");
            }
        }
        Err(e) => {
            eprintln!("Ошибка : {}", e);
            std::process::exit(1);
        } //test for git
    }
}
