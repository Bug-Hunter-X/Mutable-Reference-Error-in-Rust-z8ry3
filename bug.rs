fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // this is where the error is 
    *y = 6; 
    *z = 7; // this line will panic at runtime 
}