pub fn has_program(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => {
            println!("cargo:warning=Could not find invokable {}!", prog);
            false
        }
    }
}
