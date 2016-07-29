// getestet wird mit cargo test
//
// Test-Funktionen sind in src/ und tests/ möglich
// Jeder Test ohne panic!() ist OK (Test passed)

pub fn plus_zehn(a: i32) -> i32 {
    a + 10
}



#[test] // Diese Funktion wird getestet
fn it_works_assert_eq() {
        assert_eq!(15, plus_zehn(5));   // getestet wird die Funktion plus_zehn
        // assert_eq!(x,y) - 2 Dinge vergleichen
        // sind beide gleich = OK (Test passed), sind beide nicht gleich = panic!() (Test failed)
    }



// Modul für Tests anlegen, Tests gruppieren
// wird nicht ins crate kompiliert, ist nur für Tests
#[cfg(test)]
mod tests {
    use super::plus_zehn;   // Funktion plus_zehn zum Ausführen ins Modul tests importieren
    // use super::*; // alle Funktionen importieren

    #[test] // Diese Funktion wird getestet
    fn it_works_assert_eq() {
        assert_eq!(15, plus_zehn(5));  // getestet wird die Funktion plus_zehn
        // assert_eq!(x,y) - 2 Dinge vergleichen
        // sind beide gleich = OK (Test passed), sind beide nicht gleich = panic!() (Test failed)
    }

    #[test] // Diese Funktion wird getestet
    fn it_works_assert() {
        assert!(false);     // assert!() - kein true führt zu einem panic!() (Test failed)
    }

    #[test] // Diese Funktion wird getestet
    #[should_panic]  // Bei panic!() ist der Test OK (Test passed)
    fn it_works_assert_should_panic() {
        assert!(false);     // assert!() - kein true führt zu einem panic!() (Test failed)
    }

    #[test] // Diese Funktion wird getestet
    #[ignore] // Diesen Test ignorieren (Test ignored)
    fn it_works_assert_eq_ignore() {
        assert_eq!(15, plus_zehn(5));  // getestet wird die Funktion plus_zehn
        // assert_eq!(x,y) - 2 Dinge vergleichen
        // sind beide gleich = OK (Test passed), sind beide nicht gleich = panic!() (Test failed)
    }
    // cargo test -- --ignored  // nur ignore-Tests werden durchgeführt

}




