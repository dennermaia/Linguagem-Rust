fn main() {
    let term = 9;
    let mut cont = 1;
    let mut old = 0;
    let mut new = 1;
    let mut fibonacci = 1;

    while cont <= term {
    
    print!("{}", fibonacci);
    
    if cont < term {
        print!(", ");
    }
        
    new = new + old;
    old = new - old;       

    cont += 1;

    fibonacci = new;

    }
    print!(", ...");
    println!("");
}
