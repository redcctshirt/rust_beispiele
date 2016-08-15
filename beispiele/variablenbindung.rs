fn main() {

    // let - Variablenbindung erstellen, Standard: immutable (Wert ist unveränderbar)
    let a = 1;
    println!("a = {}", a);

    // Muster sind bei einer Variablenbindung möglich, z.B.
    let (b, c, d) = (2, 3, 4);
    println!("b = {}, c = {}, d = {}", b, c, d);
    
    // Variablenbindung mit Typangabe
    // ohne Typangabe versucht Rust den Typ alleine herauszufinden
    // i - vorzeichenbehaft u - vorzeichenlos
    // Ganzzahlen: 8, 16, 32, 64 Bits
    let e: i32 = 5;
    println!("e = {}", e);

    // mut - mutable (Wert ist veränderbar)
    let mut f = 1;
    f = 6;
    println!("f = {}", f);
}
