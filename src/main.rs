use std::process::{Command, exit};

// fn pull() {
//     let pull = Command::new("git")
//     .arg("pull").arg("origin").arg("master").output()
//     .expect("failed to pull repo");

//     if !pull.status.success() {
//         eprintln!("Error: Failed to pull repo");
//         exit(1);
//     }
// }

fn commit_push() {
    let add = Command::new("git")
        .arg("add").arg(".").output()
        .expect("failed to git add");

    if !add.status.success() {
        eprintln!("Error: Failed to add files: {:?}", String::from_utf8_lossy(&add.stderr));
        exit(1);
    }

    let commit = Command::new("git")
        .arg("commit").arg("-m").arg("update repo").output()
        .expect("failed to git commit");

    if !commit.status.success() {
        eprintln!("Error: Failed to commit files: {:?}", String::from_utf8_lossy(&commit.stderr));
        exit(1);
    }

    let push = Command::new("git")
        .arg("push").arg("origin").arg("master").output()
        .expect("failed to git push");

    if !push.status.success() {
        eprintln!("Error: Failed to push files: {:?}", String::from_utf8_lossy(&push.stderr));
        //pull();
        exit(1);
    }

    println!("Successfully pushed files to git repo!");
}

fn main() {
    commit_push();
}
