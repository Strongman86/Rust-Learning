pub mod outer_mod{
    pub(self) fn outer_mod_fn(){}
    pub mod inner_mod{
        use crate::outer_mod::outer_mod_fn;
        pub(in crate:: outer_mod) fn outer_mod_visible_fn(){
            println!("test");
        }

        pub(crate) fn crate_visible_fn(){}

        pub(super) fn super_mod_visible_fn(){
            inner_mod_visible_fn();
            outer_mod_fn();
        }
        pub(self) fn inner_mod_visible_fn(){}
    }
    pub fn foo(){
        inner_mod::outer_mod_visible_fn();
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();
        //inner_mod::inner_mod_visible_fn();
    }
}
fn bar(){
    outer_mod::inner_mod::crate_visible_fn();
    //outer_mod::inner_mod::super_mod_visible_fn();
    outer_mod::foo();
}
fn main() {
    bar();
    println!("Hello, world!");
}
