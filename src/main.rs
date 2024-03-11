use std::{env, fs, path::PathBuf};

static CURRENT_DIR: &str = ".m2";
static WORK_DIR: &str = ".m2.work";
static PERSONAL_DIR: &str = ".m2.personal";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("The current context is: {}", get_current_ctx())
    } else if args.len() == 2 && args[1] == "switch" {
        switch_ctx()
    } else {
        eprintln!("Usage: m2ctx [switch]")
    }
}

fn get_current_ctx() -> String {
    let home = get_home();
    let work_path = PathBuf::from(home.clone()).join(WORK_DIR);
    let personal_path = PathBuf::from(home.clone()).join(PERSONAL_DIR);

    if fs::metadata(work_path).is_ok_and(|m| m.is_dir()) {
        "personal".to_string()
    } else if fs::metadata(personal_path).is_ok_and(|m| m.is_dir()) {
        "work".to_string()
    } else {
        "unknown".to_string()
    }
}

fn switch_ctx() {
    let home = get_home();
    let curr_ctx = get_current_ctx();
    let work_path = PathBuf::from(home.clone()).join(WORK_DIR);
    let personal_path = PathBuf::from(home.clone()).join(PERSONAL_DIR);
    let curr_path = PathBuf::from(home.clone()).join(CURRENT_DIR);

    if curr_ctx == "work" {
        fs::rename(curr_path.clone(), work_path).expect("Failed to rename .m2");
        fs::rename(personal_path, curr_path.clone()).expect("Failed to rename .m2.personal");
        println!("Switched to personal context")
    } else if curr_ctx == "personal" {
        fs::rename(curr_path.clone(), personal_path).expect("Failed to rename .m2");
        fs::rename(work_path, curr_path.clone()).expect("Failed to rename .m2.personal");
        println!("Switched to work context")
    } else {
        eprintln!("Fatal error: Unknown context!")
    }
}

fn get_home() -> String {
    env::var("HOME").expect("Failed to get HOME directory")
}
