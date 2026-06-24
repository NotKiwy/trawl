use colored::Colorize;

pub fn __logo() {
    let _logo = "  ______                    __\n /_  __/________ __      __/ /\n  / / / ___/ __ `/ | /| / / / \n / / / /  / /_/ /| |/ |/ / /  \n/_/ /_/   \\__,_/ |__/|__/_/   ";
    println!("{}", _logo.white().bold());
    println!();
}

pub fn __ok(_msg: &str) {
    println!("{} {}", "[+]".green().bold(), _msg);
}

pub fn __err(_msg: &str) {
    println!("{} {}", "[-]".red().bold(), _msg);
}

pub fn __warn(_msg: &str) {
    println!("{} {}", "[!]".yellow().bold(), _msg);
}

pub fn __info(_msg: &str) {
    println!("{} {}", "[#]".bright_black().bold(), _msg);
}

pub fn __action(_msg: &str) {
    println!("{} {}", "[~]".cyan().bold(), _msg);
}

pub fn __cmd(_msg: &str) {
    println!("{} {}", "[>]".bright_black(), _msg.bright_black());
}

pub fn __info_str(_msg: &str) -> String {
    format!("{} {}", "[#]".bright_black().bold(), _msg)
}
