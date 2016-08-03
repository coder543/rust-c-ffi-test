extern crate gcc;

fn main() {
    gcc::compile_library("libtest.a", &["src/test.c"]);
}