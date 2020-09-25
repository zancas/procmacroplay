pub fn main() {
    procmacroplay::token_for_if!() true {
        println!("foo");
    }
}
