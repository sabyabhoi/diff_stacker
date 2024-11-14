use std::process::Command;

/*
git for-each-ref --format='%(refname:short) %(upstream:short)' refs/heads | awk -v current=$(git symbolic-ref --short HEAD) '$2 == current {print $1}'
*/

fn main() {
    match Command::new("git")
        .args([
            "for-each-ref",
            "--format='%(refname:short) %(upstream:short)'",
            "refs/head",
        ])
        .output()
    {
        Ok(o) => println!(
            "{}",
            o.stdout.iter().map(|&x| x as char).collect::<String>()
        ),
        Err(e) => println!("Error: {}", e),
    }
}
