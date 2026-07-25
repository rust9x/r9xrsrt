fn main() {
    windows_bindgen::bindgen([
        "--out",
        "src/bindings.rs",
        "--flat",
        "--sys",
        "--link",
        "crate::windows_link",
        "--filter",
        "ExitProcess",
        "HANDLE",
        "DLL_PROCESS_ATTACH",
        "DLL_PROCESS_DETACH",
        "PIMAGE_TLS_CALLBACK",
    ])
    .unwrap();
}
