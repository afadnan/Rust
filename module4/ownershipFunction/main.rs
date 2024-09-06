//OwnerShip -> use to memory management
// R1 -> Each value in Rust has a variable that's called its owner.
//R2 -> there can be only one owner at a time
//R3 -> When the owner goes out of scope,the value will be dropped.

/* 
fn main() {
    let s1 = String::from("hello");//it allocating in the heap memory
    let s2 = s1;//pointer of s1 is pointing to s2 -> now s2 is the owner of s1
    println!("{s2}");
   // println!("{s1}"); now s1 is invalid
}
*/
/* 
fn main() {
    let n1 :u8 = 23;//it is allocate in the stack memory
    let n2 = n1; // no error because n2 make a copy of n1
    println!("{n1}");
    println!("{n2}");
}
*/ 
//-----------------Functions-------------------------------
/* 
fn main() {
    let x:u8 =5;
    process_integer(x);
    println!("\nThe value of X in main () is {x}\n")
}
fn  process_integer(x:u8){
    print!("\nthe value of x in process_integer is {x}\n");
}
*/

fn main() {
    let s = String ::from ("Adnan");
    process_string(s);
   // println!("\nThe value of X in main () is {x}\n") error because now item own String ->Adnan
}
fn  process_string(item:String){
    print!("\nthe value of s in process_string is {item}\n");
}




