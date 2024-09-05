fn main() {
    /* 
    let num:u16 = 65535;
    println!("the value stored in num is {}",num);
    
    let num = 65500;
    println!("the value stored in num is {}",num);
    */
    //&str -> by default-> not mut -> fixed length string -> not stored in heap/may be stack or some special memory when only we can read
    /* 
    let string_literal:&str = "Hello Adnan, How are you doing";
    println!("This is &str literal {}",string_literal);
    //string->mut -> Dynamic Length String -> Stored in heap 
    let mut strings_literal:String = String::from( "Hello Adnan, How are you doing");
    strings_literal.push_str("All good ?");
    println!("This is String literal {}",strings_literal)
    */
    //Tuple -> store multiple data types in the same variable
    let emp_info:(&str,u8) = ("Adnan",23);

    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    println!("Employee Name is {} and its age is {}",emp_name,emp_age);
    //destructing
    let (employee_name,employee_age) = emp_info;
    println!("Employee Name is {} and its age is {}",employee_name,employee_age);
}
