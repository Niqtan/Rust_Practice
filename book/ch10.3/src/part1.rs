fn main() {
    let r; // 'a

    {
        let x = 5; // -+-- 'b
        r = &x;
        //Danging Reference because it points to
        //invalid memory after the scope
    } // +

    println!("r: {}", r);
}

//Example 02 (Valid lifetimes)
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                                      +