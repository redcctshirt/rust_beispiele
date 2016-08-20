fn main() {
    
    // Schleifen - loop, for, while

    // loop (ohne Abbruch -> Endlosschleife)
    // break - Abbruch der Schleife, continue - aktuellen Durchlauf abbrechen
    let mut a = 0;
    loop {
        a = a + 1;
        println!("loop: {}", a);
        if a == 5 { break }
    }

    // while (Sobald Bedingung erfüllt ...)
    let mut b = 0;
    while b != 5 {
        b += 1;
        println!("while: {}", b);
    }

    // for-Schleife
    for c in 1..6 {
        println!("for: {}", c); 
    }
    
    // mit enumerate() können die Durchläufe gezählt werden (bei 0 geht es los!)
    for (d,e) in (10..16).enumerate() {
        println!("for mit enumerate: {}, Durchlauf: {}", e, d);
    }
    
}
