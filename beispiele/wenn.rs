fn main() {
    
    // if - Wenn Bedingung erfüllt dann ...
    let a = true;
    if a {
        println!("a = {}", a);
    }    
       
    // if, else if, else
    let b = 1;
    if b == 0 {
        println!("b = 0");
    } else if b == 1 {
        println!("b = 1");
    } else {
        println!("b = {}", b);
    }

    // if ist ein Ausdruck
    let c = if b == 1 { "Hallo" } else { "Hi" };
    println!("c = {}", c);

    // == != < > <= >=  !
    // && (und) || (oder) 
    let d = 10;
    if !(a == false) && d >= 5 || d < 15 {
        println!("True");
    }

}
