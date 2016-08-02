//! //! ist ein Dokumentationskommentar für Module (für den äußeren Codeblock)
//! # Examples
//! 
//! ```
//! println!("Hello, world!");
//! ```
//! 

/// /// ist ein Dokumentationskommentar für die Dokumentation
/// 
/// rustdoc und cargo doc generieren die Dokumentation, 
/// der Dokumentationskommentar wird in Markdown (md) geschrieben
///
/// # Examples
/// 
/// ```
/// ausgabe();
/// ```
/// 
/// spezielle Überschriften: /// # Panics /// # Errors /// # Safety /// # Examples
///
/// Funktion ausgabe() erzeugt die Ausgabe Hello, world!
pub fn ausgabe() {
   println!("Hello, world!"); 
}

/// Funktion main() führt die Funktion ausgabe() aus
pub fn main() {
    ausgabe();
}
