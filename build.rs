fn main() {
    // Windows環境の場合のみ、Rstrtmgr.lib をリンクするように指示
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=rstrtmgr");
    }
}
