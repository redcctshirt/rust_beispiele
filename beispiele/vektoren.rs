fn main() {

    // Vektoren Vec<T> - eine dynamische Liste, 

    // leeren Vektor für i32-Werte erstellen
    let mut a: Vec<i32> = Vec::new();
    a.push(1);  // push - am Ende Wert hinzufügen, Wert 1 - Index 0 hinzufügen
    a.push(2);  // Wert 2 - Index 1 hinzufügen  

    println!("a: {}", a[1]);    // Ausgabe Index 1 - Wert 2

    // Vektoren können ebenfalls mit dem Makro vec! erzeugt werden
    let b = vec![1, 2, 3];
    let mut b_ = vec![1, 2, 3, 4, 5, 6];
    let c = vec![1; 5];   // 5x1 = [1,1,1,1,1] 
    let b_pop = b_.pop();   // pop - letzten Wert vom Vektor abholen und aus der Vektorliste streichen
    
    println!("b: {}", b[2]);
    println!("c: {}", c[4]);
    println!("b: pop: {:?} {}", b_pop, b_.len());   // len - Anzahl der Werte in der Vektorliste

    // für den Index müssen Variablen mit dem Typ usize verwendet werden, i32 führt zu einem Fehler
    let d: usize = 3;
    println!("d: {}", c[d]);

    // matchen um zu prüfen ob es diesen Index gibt, mit get() oder get_mut()
    match b.get(2) {
        Some(e) => println!("b: Index 2 hat den Wert {}", e),
        None => println!("Diesen Index gibt es nicht !")
    }

    // Vektor mit for-Schleife durchlaufen
    for f in &b {
        println!("b: Wert: {}", f);
    }

}
