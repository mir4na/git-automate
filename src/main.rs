use std::process::{Command, exit};
// use names::Generator;

fn commit_push() {
    let add = Command::new("git")
        .arg("add").arg(".").output()
        .expect("failed to git add");

    if !add.status.success() {
        eprintln!("Error: Failed to add files");
        exit(1);
    }

    //let commit_message = generator();
    let commit = Command::new("git")
        .arg("commit").arg("-m").arg("commit files").output()
        .expect("failed to git commit");

    if !commit.status.success() {
        eprintln!("Error: Failed to commit files");
        exit(1);
    }

    let push = Command::new("git")
        .arg("push").arg("origin").arg("master").output()
        .expect("failed to git push");

    if !push.status.success() {
        eprintln!("Error: Failed to push files");
        exit(1);
    }

    println!("Successfully pushed files to git repo!");
}

// fn generator() -> String {
//     let mut generator = Generator::default();
//     generator.next().unwrap()
// }

fn main() {
    commit_push();
}
