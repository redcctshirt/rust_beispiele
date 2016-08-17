// einfache Funktionsdeklaration
fn plus_eins() {
    println!("plus_eins: ");
}

// Funktionsdeklaration mit einem Argument
fn plus_zwei(a: i32) {
    println!("plus_zwei: {}", a);
}

// Funktionsdeklaration mit zwei Argumenten
fn plus_drei(a: i32, b: i32) {
    println!("plus_drei: {} + {} = {}", a, b, a + b);
}

// Funktionsdeklaration mit Rückgabe (-> Typ)
// die letzte Zeile enthält den Wert für die Rückgabe, ohne ; (es ist ein Ausdruck, keine Anweisung)
fn plus_vier(a: i32) -> i32 {
    a + 4
}

// Funktionsdeklaration mit Rückgabe vor der letzten Zeile
// #[allow(unreachable_code)] - Warnung "unreachable_code" ignorieren, nicht anzeigen
#[allow(unreachable_code)]
fn plus_fuenf(a: i32) -> i32 {
    return a;    
    a + 5
}

// wird automatisch ausgeführt
fn main() {
    plus_eins();
    plus_zwei(2);
    plus_drei(3,4);
    println!("plus_vier: {}",plus_vier(5));
    println!("plus_fuenf: {}",plus_fuenf(6));

    // Funktionszeiger auf Funktion plus_zwei erstellen, mit Typinferenz 
    // Funktion plus_zwei durch Funktionszeiger f1 ausführen
    let f1 = plus_zwei;
    f1(7);

    // Funktionszeiger auf Funktion plus_vier erstellen, ohne Typinferenz 
    // Funktion plus_vier durch Funktionszeiger f2 ausführen
    let f2: fn(i32) -> i32 = plus_vier;
    println!("f2 - plus_vier: {}",f2(8));
}
