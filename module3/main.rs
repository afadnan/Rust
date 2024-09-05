//Functions

/* 
fn main() {
    let num_1 :u8 = 23;
    let num_2 :u8 = 16;
    add(num_1,num_2);
}
fn add(item_1:u8,item_2:u8){
    let result :u8;
    result = item_1 + item_2;
    println!("{}",result);
}
*/
fn main () {
    let num_1 :u8 = 23;
    let num_2 :u8 = 16;
    let result :u8 = add(num_1,num_2);
    println!( "{}", result);
}
fn add(item1:u8,item2:u8)->u8{
    return item1+item2;
}