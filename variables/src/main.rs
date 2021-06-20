fn main() {
    // Explict
    let x: i32;
    x = 9;

    // Inline
    let y: i32 = 33;

    // Inference
    let z = 69;
    print!("{}\n{}\n{}\n", x, y, z);

    // Strings
    // Inference
    // Is allocted in Stack memory
    let str = "Hello world";

    // String slice
    let strpointer: &str = "Pointer String";

    // String  
    let str_string: String = "String".to_string();


    print!("{}\n{}\n{}\n", str,strpointer, str_string)
}
