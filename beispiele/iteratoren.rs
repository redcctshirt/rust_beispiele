// Iteratoren

fn main() {
  
    // 0..5 ist ein Range (Bereich von x bis y)
    // Bereiche sind Iteratoren
    // man kann darauf .next() aufrufen um den nächsten Eintrag in der Reihenfolge aufzurufen
  
    println!("\nfor a in 0..5");
    for a in 0..5 {
        print!("{} ", a);
    }
    // 0 1 2 3 4



    // veränderbare Variable für Bereich 0..6 definieren
    let mut bereich = 0..6;

    // Schleife starten
    println!("\n\nloop match 0..6.next()");
    loop {
        match bereich.next() {  // eins weiter im Bereich
            Some(a) => {
                print!("{} ", a);
            },
            None => { break }  // kein Eintrag mehr - Schleife abbrechen
        }
    }
    // 0 1 2 3 4 5



    // Vektoren haben eigene Iteratoren
    let vektor = vec![1, 2, 3, 4, 5, 6];

    println!("\n\nfor v in vec![1,2,3,4,5,6]");
    for v in &vektor {
        print!("{} ", *v);
    }
    // 1 2 3 4 5 6



    // collect ist ein Konsument, Konsumenten erstellen aus Iteratoren Mengen oder Werte
    let c = (0..6).collect::<Vec<i32>>();

    println!("\n\nfor a in 0..6.collect::<Vec<i32>>()");
    for a in c {
        print!("{} ", a);
    }
    // 0 1 2 3 4 5


    
    // find ist ein Konsument, findet das erste Element auf das x > 5 zutrifft und gibt Option<T> zurück
    let f = (0..10).find(|a| *a > 5);

    println!("\n\nmatch f (0..10).find(|a| *a > 5)");
    match f {
        Some(b) => println!("{} ",b),
        None => println!("Nix gefunden."),
    }
    // 6


    
    // Iterator-Adapter verändern Iteratoren und erstellen daraus neue Iteratoren
    // map ist ein Iterator-Adapter, alle Zahlen werden um 1 erhöht, 0 1 2 3 4 wird zu 1 2 3 4 5   
    let m = (0..5).map(|a| a + 1);

    println!("\nfor i in (0..5).map(|a| a + 1)");
    for i in m {
        print!("{} ", i);
    }
    // 1 2 3 4 5

    
    
    // take ist ein Iterator-Adapter, x Elemente werden zurückgegeben, 0.. bedeutet unendlich
    println!("\n\nfor t in (0..).take(4)");
    for t in (0..).take(4) {
        print!("{} ", t);
    }
    // 0 1 2 3


    
    // filter ist ein Iterator-Adapter, filtern - nur Werte zurückgeben die bei der Bedingung wahr sind
    println!("\n\nfor f in (0..10).filter(|&a| a % 2 == 0)");
    for f in (0..10).filter(|&a| a % 2 == 0) {
        print!("{} ", f);
    }
    // 0 2 4 6 8

    
    
    // Kombinationen sind möglich
    println!("\n\nfor f in (0..20).filter(|&a| a % 2 == 0).take(6)");
    for f in (0..20).filter(|&a| a % 2 == 0).take(6) {
        print!("{} ", f);
    }
    // 0 2 4 6 8 10

}




