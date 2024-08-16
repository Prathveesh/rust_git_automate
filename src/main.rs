use std::process::{Command, exit};

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Fail in adding files to the local repo.");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Commiting latest changes")
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("Error: Fail in committing changes.");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("Error: Fail in pushing changes to remote.");
        exit(1);
    }

    println!("Changes are successfully added, committed, and pushed.");
}

fn main() {
    update_commit_push();
}