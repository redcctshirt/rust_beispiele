// cargo run --features "en" - Testen mit Feature en
// cargo run --features "de" - Testen mit Feature de

// bedingt Kompilieren
// #[cfg(feature = "en")] - Feature (Schalter) konfigurieren, Name kann selbst bestimmt werden
// Dieses Feature muss in Cargo.toml unter [features] aktiviert werden
// 
// Cargo.toml
// [features]
// de = []
// en = []
//

// dies wird kompiliert mit cargo build --features "en"
#[cfg(feature = "en")]
fn main() {
    println!("Hello world!");
    main_os();
}

// dies wird kompiliert mit cargo build --features "de" 
#[cfg(feature = "de")]
fn main() {
    println!("Hallo Welt!");
    main_os();
}



// Kompilieren anhand vorhandener Konfigurationsvariablen z.B. target_os

// nur kompilieren wenn target_os ein Linux-Betriebssystem ist
#[cfg(target_os = "linux")]
fn is_linux() {
    println!("Es ist Linux.")
}

// nur kompilieren wenn target_os kein Linux-Betriebssystem ist
#[cfg(not(target_os = "linux"))]
fn is_linux() {
    println!("Es ist kein Linux.")
}

fn main_os() {
    is_linux();

    // Nur ausführen wenn Bedingung mit vorhandener Konfigurationsvariable stimmt 
    if cfg!(target_os = "linux") {
        println!(":-)");
    } else {
        println!(":-(");
    }
}
