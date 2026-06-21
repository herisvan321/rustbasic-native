use rustbasic_core::colored::Colorize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "install" => {
            rustbasic_native::scaffolding::make_native_scaffolding();
        }
        "uninstall" => {
            rustbasic_native::scaffolding::remove_native_scaffolding();
        }
        _ => {
            println!("{} {}", "❌ Error: Perintah tidak dikenal:".red().bold(), args[1].yellow());
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "📱💻 RustBasic Native CLI".magenta().bold());
    println!("{}", "==============================".magenta());
    println!("{}", "Usage:".bold());
    println!("  rustbasic-native install    {}", "Scaffold Native Android, iOS & Desktop wrappers into your project".dimmed());
    println!("  rustbasic-native uninstall  {}", "Clean up all native scaffolding files from your project".dimmed());
    println!();
}
