extern crate cc;

use std::path::Path;

fn main() {
    let xmss_dir = Path::new("xmss-reference");
    let xmss_files = vec![
        xmss_dir.join("fips202.c"),
        xmss_dir.join("hash_address.c"),
        xmss_dir.join("hash.c"),
        xmss_dir.join("params.c"),
        xmss_dir.join("randombytes.c"),
        xmss_dir.join("utils.c"),
        xmss_dir.join("wots.c"),
        xmss_dir.join("xmss.c"),
        xmss_dir.join("xmss_commons.c"),
        //xmss_dir.join("xmss_core.c"),
        xmss_dir.join("xmss_core_fast.c"),
    ];

    cc::Build::new()
        .flag("-std=c11")
        .flag("-march=native")
        .flag("-Ofast")
        .include("xmss-reference")
        .files(xmss_files.into_iter())
        .compile("xmss");

    println!("cargo:rustc-link-lib=crypto");
}
