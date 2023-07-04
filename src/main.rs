fn print_string(s: String) { 
    println!("{}", s); 
} 

fn main() {
    let s1 = String::from("hi world");
    let s2 = s1;

    let x: i32 = 42;
    let y = String::from("Rust nice programming language");
    let z = y;
    println!(" x = {} , z ={} ",x,z);

    
    let my_string = String::from("hi everyone "); 
    print_string(my_string);

}

