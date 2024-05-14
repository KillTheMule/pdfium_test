fn main() {
        println!("cargo:rustc-link-lib=static=pdfium");
        println!("cargo:rustc-link-search=native={}", env!("CARGO_MANIFEST_DIR"));
}
