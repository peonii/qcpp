use colored::*;
use std::io::Read;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 3 {
        println!("{}", "Usage: qcpp <file to change> <output file>".yellow());
        std::process::exit(0);
    }

    let filen = &args[1];

    let mut file = std::fs::File::open(filen).unwrap();
    println!("{}", "File found!".green());

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();
    let mut new_data = String::new();

    println!("{}", "Replacing directives...".yellow());
    for mut line in lines {
        line = line.trim_start();
        if line.starts_with("%incl") {
            let lib = line.split_whitespace().nth(1).unwrap().to_string().parse::<String>().unwrap();
            let mut lib_path = home::home_dir().unwrap();
            lib_path.push("q_libs");
            lib_path.push(lib);
            lib_path.set_extension("qlib");
            println!("Adding library: {}", lib_path.to_str().unwrap().yellow());
            let mut lib_file = std::fs::File::open(lib_path).unwrap();
            
            let mut lib_contents = String::new();
            lib_file.read_to_string(&mut lib_contents).unwrap();

            let lib_lines = lib_contents.lines();
            for lib_line in lib_lines {
                new_data.push_str(lib_line);
                new_data.push_str("\n");
            }
        } else {
            new_data.push_str(line);
            new_data.push_str("\n");
        }
    }
    println!("{}", "Compiling complete!".green());
    std::fs::File::create(&args[2]).unwrap().write_all(new_data.as_bytes()).unwrap();
    println!("{}{}", "Saved to file ".green(), args[2].white());

}
