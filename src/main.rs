use std::env::args;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    assert!(args.len() >= 2);

    let stacks_dir = Path::new(&args[0]);
    assert!(stacks_dir.is_dir());

    for image in &args[1..] {
        let project = image.split("/").next().unwrap();
        let config_file = find_config_file(stacks_dir, project).unwrap();

        Command::new("docker-compose")
            .args(["-p", project, "-f", config_file.to_str().unwrap(), "pull"])
            .status()
            .unwrap();
        Command::new("docker-compose")
            .args([
                "-p",
                project,
                "-f",
                config_file.to_str().unwrap(),
                "up",
                "-d",
            ])
            .status()
            .unwrap();
    }
}

fn find_config_file(dir: &Path, project: &str) -> Option<PathBuf> {
    let yml = dir.join(format!("{}.yml", project));
    if yml.is_file() {
        return Some(yml);
    }
    let yaml = dir.join(format!("{}.yaml", project));
    if yaml.is_file() {
        return Some(yaml);
    }
    None
}
