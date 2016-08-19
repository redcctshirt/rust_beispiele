## Rust (Programmiersprache) - Beispiele

https://www.rust-lang.org/ - Rust    
https://crates.io/ - Webseite für crates

* [SolidOak - IDE für Rust](https://github.com/oakes/SolidOak)
* [Editor Configs](https://github.com/rust-lang/rust/blob/master/src/etc/CONFIGS.md)
* [Rust ausprobieren](https://play.rust-lang.org/)

### Lernen

* [Die Programmiersprache Rust](https://rust-lang-de.github.io/rustbook-de/index.html)
* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust by Example](http://rustbyexample.com/index.html)
* [rust-learning](https://github.com/ctjhoa/rust-learning)

### Kommandos

```
// Version anzeigen
rustc --version 

// Kompilieren: main.rs -> ./main
rustc main.rs

// Dokumentation generieren
rustdoc

// Cargo - ist das zentrale Verwaltungstool für Rust-Projekte
//
// neues bin-Projekt erstellen -> Cargo.toml, src/main.rs, .git, .gitignore
// Cargo.toml - ist die Konfigurationsdatei für Projekte
cargo new neues_projekt --bin

// neues lib-Projekt erstellen -> Cargo.toml, src/lib.rs, .git, .gitignore
cargo new neues_lib_projekt

// Projekt kompilieren
cargo build

// Projekt kompilieren und starten
cargo run

// alle Abhängigkeiten aktualisieren
cargo update

// Tests durchführen
cargo test

// Dokumentation generieren
cargo doc

```


### Beispiele

* [Hallo Welt](beispiele/hallowelt.rs)
* [cargo new neues_projekt --bin](beispiele/neues_projekt)
* [cargo new neues_lib_projekt](beispiele/neues_lib_projekt)
* [Tests durchführen](beispiele/plus_x)
* [Kompilieren mit Bedingung](beispiele/feat)
* [Dokumentation im Quellcode](beispiele/doku)
* [Variablenbindung - let](beispiele/variablenbindung.rs)
* [Funktionen - fn](beispiele/funktionen.rs)
* [einfache Datentypen](beispiele/datentypen.rs)
* [Listen - Array, Slice, Tupel](beispiele/listen.rs)
* [if - else if - else](beispiele/wenn.rs)
* [Iteratoren](beispiele/iteratoren.rs)
* Nebenläufigkeit

### Lizenz

Lizenz: https://creativecommons.org/publicdomain/zero/1.0/deed.de
