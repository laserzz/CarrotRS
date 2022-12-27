use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut git;

    if args[1] == "help" {
        println!("carrotrs <path> <extension>");
        println!("args:\n--github - creates additional git-related files");
        return;
    }

    if args.contains(&"--github".to_string()) {
        git = true;
    } else {
        git = false;
    }

    make_proj(&args[1], &args[2], git);
}

#[allow(unused_must_use)]
#[allow(unused_variables)]
fn make_proj(path: &String, ext: &String, git: bool) {
    let new_path = format!("{}/src", path);
    let other_path = format!("{}", new_path);
    let filedir = format!("{}/main.{}", new_path, ext);
    println!("{}", filedir);
    println!("{}", new_path);
    fs::create_dir_all(new_path).expect("Failed to create");
    fs::File::create(filedir).expect("Failed to create");
    println!("base files made!");

    if git == true {
        let readme = format!("{}/README.md", other_path);
        fs::File::create(readme);
        let ignore = format!("{}/.gitignore", other_path);
        fs::File::create(ignore);
        println!("GitHub Files made!");
    }
}