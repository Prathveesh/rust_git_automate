use std::process::{Command, exit};

struct Generator {
    // Define any fields you need here, if any
}

impl Generator {
    pub fn default() -> Self {
        Generator {
            // Initialize fields if necessary
        }
    }

    pub fn next(&mut self) -> Option<String> {
        Some("Generated commit message".to_string())
    }
}

fn update_commit_push() {
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the local repo.");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_gen())
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes.");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes to remote.");
        exit(1);
    }

    println!("Changes are successfully added, committed, and pushed.");
}

fn name_gen() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push();
}
