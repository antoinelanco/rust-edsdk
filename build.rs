fn main() {
    println!("cargo:rustc-link-search=native=native");
    println!("cargo:rustc-link-lib=dylib=EDSDK");

    // // Génération des bindings avec bindgen
    // let bindings = bindgen::Builder::default()
    //     .header("native/EDSDK.h")
    //     .generate()
    //     .expect("Échec de la génération des bindings");

    // let out_path = std::path::PathBuf::from("src");
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Échec d'écriture des bindings");
}
