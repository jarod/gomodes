use std::env;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let dir = std::env::current_dir()?;

    for entry in walkdir::WalkDir::new(dir) {
        let entry: walkdir::DirEntry = entry?;
        let path: &std::path::Path = entry.path();
        if !path.is_file() || path.file_name().unwrap() != "go.mod" {
            continue;
        }
        println!("{:?}", path);
        let mut args = env::args();
        args.next();
        let mut cmd = Command::new(args.next().expect("command"));
        cmd.current_dir(path.parent().unwrap());
        cmd.args(args);
        println!("{:?}", cmd);

        let output = cmd.output()?;
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    }
    Ok(())
}
