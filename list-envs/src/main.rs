use colored::Colorize;

fn main() {

    println!("{}", "Environment Variables".green());

    let vars = std::env::vars();

    for var in vars {
        println!("{}: {}", var.0, var.1);
    }

    println!("{}", "Environment OS Variables".green());

    let os_vars = std::env::vars_os();

    for os_var in os_vars {
        println!("{}: {}", os_var.0.into_string().unwrap(), os_var.1.into_string().unwrap());
    }

    println!("{}", "Current Directory".green());

    let cur_dir = std::env::current_dir();

    println!("{}", cur_dir.unwrap().as_path().display());

    println!("{}", "Current Executable Path".green());

    let exe_dir = std::env::current_exe();

    println!("{}", exe_dir.unwrap().as_path().display());
}
