#![feature(proc_macro)]
extern crate the_macro;
use the_macro::lisp_fn;
#[lisp_fn]
fn the_value() -> u8 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(the_value(), 255);
    }
}
