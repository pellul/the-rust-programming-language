fn main() {
    mutability();
    constants();
    shadowing();
}

fn mutability() {
    let x = 5;
    println!("{}", x);
    // The following line prevents the code from compiling
    // since x is not mutable
    // x = 6;

    let mut y = 6;
    println!("{}", y);
    // Since y is mutable, it's possible to give it a new value
    y = 7;
    println!("{}", y);
}


fn constants() {
    /* Constant in Rust:
     * - keyword is const (instead of let)
     * - always immutable (can't use `mut` with constants)
     * - the type MUST be specified
     * - name SHOULD be upper case
     * To have a better understanding on constant evaluation, see:
     * https://doc.rust-lang.org/reference/const_eval.html
     */

    const HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", HOURS_IN_SECONDS);
    let x = HOURS_IN_SECONDS * 3;
    println!("{} seconds in 3 hours", x);
}


fn shadowing() {
    /* Shadowing is declaring an immutable variable a new.
     * To do so, it is necessary to use the let keyword
     * It makes a immutable variable to be edited without changing its
     * immutability.
     * The type of the variable can move upon shadowing according to the value
     * given.
     */

    // Example: &str becomes a usize
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{} spaces", spaces);
    // The following lines, on the other hand, fail on compilation
    // let mut aspaces = "   ";
    // aspaces = aspaces.len();

    // To get the same output without shadowing, we need to declare two variables
    let spaces_str = "    ";
    let spaces_num = spaces_str.len();
    println!("{} spaces", spaces_num);
}
