const OUT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/bindings.rs");

fn main() {
    windows_bindgen::builder()
        .output(OUT)
        .flat()
        .sys()
        .filters([
            "ExitProcess",
            "HANDLE",
            "DLL_PROCESS_ATTACH",
            "DLL_PROCESS_DETACH",
            "PIMAGE_TLS_CALLBACK",
        ])
        .write();

    let bindings = std::fs::read_to_string(OUT).unwrap();
    std::fs::write(
        OUT,
        format!("#![allow(nonstandard_style, clippy::all)]\n\nuse super::windows_link;\n\n{bindings}"),
    )
    .unwrap();
}
