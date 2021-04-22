pub fn has_prog(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => {
            if out.stdout.len() > 0 {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
