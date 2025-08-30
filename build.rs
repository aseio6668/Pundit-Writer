fn main() {
    // Set stack size to 16MB on Windows to handle deep recursion in contemplative writing system
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg=/STACK:16777216"); // 16MB stack size
    }
    
    // On other platforms, we can use environment variables at runtime
    #[cfg(not(target_os = "windows"))]
    {
        // Unix systems will use RUST_MIN_STACK environment variable
        println!("cargo:rerun-if-env-changed=RUST_MIN_STACK");
    }
}