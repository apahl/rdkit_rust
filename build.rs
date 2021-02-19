extern crate cpp_build;

fn main() {
    let rdkit_conda = std::env::var("RDKIT_CONDA")
    .expect("RDKIT_CONDA environment variable not set. \nPlease set to the location of your conda RDKit installation, e.g. `/home/user/anaconda3/envs/chem`");
    let include_path_1 = format!("{}/include", rdkit_conda);
    let include_path_2 = format!("{}/include/rdkit", rdkit_conda);
    let lib_path = format!("{}/lib", rdkit_conda);
    cpp_build::Config::new()
        .include(include_path_1)
        .include(include_path_2)
        .build("src/lib.rs");
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=RDKitGraphMol");
    println!("cargo:rustc-link-lib=RDKitSmilesParse");
    println!("cargo:rustc-link-lib=RDKitDescriptors");
    println!("cargo:rustc-link-lib=RDKitSubstructMatch");
    println!("cargo:rustc-link-lib=RDKitChemTransforms");
    // println!("cargo:rustc-link-lib=RDKitRDGeneral");
}
