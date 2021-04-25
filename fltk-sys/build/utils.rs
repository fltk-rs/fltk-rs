pub fn has_prog(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => {
            !out.stdout.is_empty()
        }
        _ => false,
    }
}
