use eyre::Result;
use std::process::Command;

pub fn get_branches() -> Result<Vec<(String, Option<String>)>> {
    let branch_vec_raw = Command::new("git")
        .args([
            "for-each-ref",
            "--format=%(refname:short) %(upstream:short)",
            "refs/heads",
        ])
        .output()?
        .stdout;

    let edges: Vec<(String, Option<String>)> = String::from_utf8(branch_vec_raw)?
        .lines()
        .into_iter()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let branch = iter.next().unwrap().to_string();
            let upstream = iter.next().map(|s| s.to_string());
            (branch, upstream)
        })
        .collect();

    Ok(edges)
}

pub fn create_new_stacked_branch(s: &String) -> Result<()> {
    let current_branch_raw = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("failed to get current branch")
        .stdout;
    let current_branch = String::from_utf8(current_branch_raw)?.trim().to_string();

    Command::new("git")
        .args(["checkout", "-b", s])
        .output()
        .expect("failed to create new branch");

    Command::new("git")
        .args(["branch", "--set-upstream-to", &current_branch])
        .spawn()?;

    Ok(())
}
