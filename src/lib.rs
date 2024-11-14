mod graph;

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
