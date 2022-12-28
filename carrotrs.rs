use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().len() < 3 && args[1] != "help" {
        panic!("invalid number of arguments");
    }

    let mut git = true;

    if args[1] == "help" {
        println!("carrotrs <path> <extension>");
        println!("args:\n--github - creates additional git-related files");
        return;
    }

    if !args.contains(&"--github".to_string()) {
        git = false;
    }

    make_proj(&args[1], &args[2], git);
}

#[allow(unused_must_use)]
fn make_proj(path: &String, ext: &String, git: bool) {
    let new_path = format!("{}/src", path);
    let filedir = format!("{}/main.{}", &new_path, ext);
    println!("{}", filedir);
    println!("{}", &new_path);
    fs::create_dir_all(&new_path).expect("Failed to create");
    fs::File::create(filedir).expect("Failed to create");
    println!("base files made!");

    if git == true {
        let readme_data = "# Project Name!\n\nCool Project Description";
        let readme = format!("{}/README.md", &new_path);
        fs::File::create(&readme);
        fs::write(readme, readme_data).expect("Failed to write");
        let ignore = format!("{}/.gitignore", &new_path);
        fs::File::create(ignore);
        println!("GitHub Files made!");
    }
}