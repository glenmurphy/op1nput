fn main() {
    #[cfg(target_os = "windows")]
    windres::Build::new().compile("op1nput.rc").unwrap();
}
