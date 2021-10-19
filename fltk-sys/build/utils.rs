pub fn has_program(prog: &str) -> bool {
    match std::process::Command::new(prog).arg("--version").output() {
        Ok(out) => !out.stdout.is_empty(),
        _ => false,
    }
}

pub fn cmake_version() -> Result<(u8, u8, u8), Box<dyn std::error::Error>> {
    let cmd = std::process::Command::new("cmake").arg("--version").output()?;
    let version = String::from_utf8_lossy(&cmd.stdout);
    let version: Vec<&str> = version.split_whitespace().collect();
    let version: Vec<&str> = version[2].split('.').collect();
    let mut temp = vec![];
    for i in version.iter() {
        let curr = i.parse()?;
        temp.push(curr);
    }
    Ok((temp[0], temp[1], temp[2]))
}

pub fn cmake_has_parallel() -> bool {
    if let Ok(cmake_version) = cmake_version() {
        if (cmake_version.0 > 3) || (cmake_version.0 == 3 && cmake_version.1 >= 12) {
            true
        } else {
            false
        }
    } else {
        false
    }
}