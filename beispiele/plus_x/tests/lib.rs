// getestet wird mit cargo test
//
// im Verzeichnis tests werden Integrationstests durchgeführt
extern crate plus_x;    // die Bibliothek plus_x wird eingebunden


#[test] // Diese Funktion wird getestet
fn it_works_tests() {
        assert_eq!(15, plus_x::plus_zehn(5));   // getestet wird die Funktion plus_zehn
        // 2 Dinge vergleichen, sind beide gleich = OK (Test passed), sind beide nicht gleich = panic!() (Test failed)
    }



