

// Fichier : build.rs
use std::env;

fn main() {
    // --- Étape 1: On compile toujours notre code C ---
    cc::Build::new()
        .file("src/math.c") // Utilisons le chemin simple pour ce test
        .compile("math_ex3"); // La librairie s'appelle toujours libmath_ex3.a

    // --- Étape 2: On donne les instructions à Cargo MANUELLEMENT ---

    // On récupère le chemin du dossier où la librairie a été créée.
    let out_dir = env::var("OUT_DIR").unwrap();

    // On dit explicitement à rustc où chercher les librairies natives.
    println!("cargo:rustc-link-search=native={}", out_dir);

    // On dit explicitement à rustc de lier notre librairie statique `math_ex3`.
    // C'est la ligne la plus importante.
    println!("cargo:rustc-link-lib=static=math_ex3");

    println!("cargo:warning=Instructions de liaison envoyées MANUELLEMENT.");
}