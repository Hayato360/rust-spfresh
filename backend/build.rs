use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let backend_path = PathBuf::from(&manifest_dir);
    let spfresh_path = PathBuf::from(&manifest_dir).join("..").join("SPFresh-main");
    
    // Tell cargo to look for shared libraries in the current backend directory first
    println!("cargo:rustc-link-search=native={}", backend_path.display());
    // Also search in the SPFresh-main directory as fallback
    println!("cargo:rustc-link-search=native={}", spfresh_path.display());
    // Add system library paths
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/usr/lib");
    println!("cargo:rustc-link-search=native=/lib");
    
    // Tell cargo to link the SPFresh C API library
    println!("cargo:rustc-link-lib=dylib=spfresh_c_api");
    
    // Tell cargo to link required system libraries
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=jemalloc");
    
    // Tell cargo to invalidate the built crate whenever the library changes
    println!("cargo:rerun-if-changed={}/libspfresh_c_api.so", backend_path.display());
    println!("cargo:rerun-if-changed={}/libspfresh_c_api.so", spfresh_path.display());
    println!("cargo:rerun-if-changed={}/spfresh_c_api.h", spfresh_path.display());
    println!("cargo:rerun-if-changed={}/spfresh_c_api.cpp", spfresh_path.display());
    
    // Set the library path for runtime
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}:$LD_LIBRARY_PATH", spfresh_path.display());
}
