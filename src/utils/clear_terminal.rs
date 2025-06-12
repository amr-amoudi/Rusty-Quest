// coped code, I have no idea how it works
pub fn clear() {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd").args(&["/C", "cls"]).status().unwrap();

    #[cfg(not(target_os = "windows"))]
    std::process::Command::new("clear").status().unwrap();
}