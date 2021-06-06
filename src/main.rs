mod submob {
    pub fn hi(){
        println!("hi:\tmodule `{}`,file `{}`",module_path!(),file!());

    }
    pub mod subsubmod1{
        pub struct Foo{
            pub  bar:i32,
        }

    }
    pub mod subsubmod{
        pub fn hey(){
            let _f = super::subsubmod1::Foo {bar :42};
            println!("hey:\tmodule `{}`,file `{}`",module_path!(),file!());
        }
    }
}
mod foo {
    pub mod bar {
        pub mod baz {
            pub fn f1() {
                println!("hello world");
            }
            fn f2() {}
            fn f3() {}
        }
    }
}

mod api {
    pub use v1::FOO;
    mod v1 {
        pub const FOO:i32 = 42;
    }
}

use foo::bar::baz;
fn main() {
    println!("module '{}`, file `{}`",module_path!(),file!());
    submob::hi();
    submob::subsubmod::hey();
    baz::f1();
    println!("This is from a private repo {:?}",api::FOO);
}
