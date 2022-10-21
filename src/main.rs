use std::process::Command;

fn run_cmd(program: &str, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    let status = cmd.status()?;

    if !status.success() {
        Err(format!("{program} {args:?} Failed"))?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cmd(
        "find",
        &vec![
            ".".to_owned(),
            "-type".to_owned(),
            "f".to_owned(),
            "-name".to_owned(),
            "*.prof*".to_owned(),
        ])?;

    Ok(())
}
