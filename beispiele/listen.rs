fn main() {
   
    // Arrays - Listen
    // [Typ; Länge der Liste]
    let a = [1, 2, 3, 4, 5];  // [i32; 5]
    // eine Liste mit 6x0
    let b = [0; 6];  // [i32; 6]

    // b[0] - ersten Wert ausgeben
    println!("Array: b[0] - {}", b[0]);

    // a.len() - Länge einer Liste ermitteln
    println!("Array: a.len() - {}", a.len());



    // Slice - Referenz auf eine Datenstruktur
    let s1 = &a[2..4];  // nur die Werte 3 und 4 vom Array a
    let s2 = &b[..];    // alle Werte vom Array b

    println!("Slice s1: {}", s1[0]);
    println!("Slice s2: {}", s2[5]);


    
    // Tupel - geordnete Liste
    let t = (1, 2, 3, "Hallo", "Welt");
    
    // t.Nr - Zugriff auf eine Stelle
    println!("Tupel: {} {}", t.3, t.4);

}
