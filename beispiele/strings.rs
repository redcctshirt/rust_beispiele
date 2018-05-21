fn main() {
    println!("Hello, world!");  // Einfache Zeichenketten-Ausgabe
    
    let mut s = String::new();              // Variable s mit leerem String erzeugen
    s = "Hallo String!".to_string();        // s mit Zeichenkette setzen
    println!("{}", s);                      // Zeichenkette ausgeben

    let s2 = "Hallo String 2".to_string(); // Zeichenkette setzen
    println!("{}", s2);                    // Ausgabe
   
    let s3 = String::from("Hallo String 3"); // Zeichenkette setzen
    println!("{}", s3);                      // Ausgabe

    let mut s4 = String::from("Hallo");	     // mutable Zeichenkette setzen
    let s4b = " String 4";                   // Zeichenkette setzen
    s4.push_str(s4b);                        // s4b zu s4 hinzufügen
    println!("{}", s4);                      // Ausgabe

    let mut s5 = String::from("Hallo");	     // mutable Zeichenkette setzen
    s5.push('5');                            // ein Char hinzufügen
    println!("{}", s5);                      // Ausgabe

    let s6 = String::from("Hallo ");       
    let s6b = String::from("String 6"); 
    println!("{}", s6 + &s6b);                // + Zeichenketten zusammenfügen

    let s7 = String::from("Hallo ");       
    let s7b = String::from("String 7"); 
    let s7c = format!("{}{}", s7, s7b);      // Zeichenketten kombinieren
    println!("{}",s7c);
	
    let len = String::from("Hallo String 8").len();  // Länge ermitteln
    println!("{}", len);

    let s9 = "Hallo String 9";
    let s9b = &s9[0..4];                     // 4 Bytes vom String, ersten 4 Zeichen
    println!("{}", s9b);

    for c in "Hallo String 10".chars() {     // Zeichen iterieren
        println!("{}", c);
    }

    for b in "Hallo String 11".bytes() {     // Bytes iterieren
        println!("{}", b);
    }
}
