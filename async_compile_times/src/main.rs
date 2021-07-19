#![recursion_limit = "256"]

use std::future::Future;

#[tokio::main]
async fn main() {
    dbg!(fn1(1).await);
}

async fn wrap<FN, FT, T>(f: FN) -> T
where
    FN: FnOnce() -> FT,
    FT: Future<Output = T>,
{
    f().await
}

macro_rules! def_fn {
    ($name:tt, $calls: tt) => {
        async fn $name<T>(t: T) -> T {
            wrap(|| async move { $calls(t).await }).await
        }
    };
}

async fn noop<T>(t: T) -> T {
    t
}

def_fn!(fn1, fn2);
def_fn!(fn2, fn3);
def_fn!(fn3, fn4);
def_fn!(fn4, fn5);
def_fn!(fn5, fn6);
def_fn!(fn6, fn7);
def_fn!(fn7, fn8);
def_fn!(fn8, fn9);
def_fn!(fn9, fn10);
def_fn!(fn10, fn11);
def_fn!(fn11, fn12);
def_fn!(fn12, fn13);
def_fn!(fn13, fn14);
def_fn!(fn14, fn15);
def_fn!(fn15, fn16);
def_fn!(fn16, fn17);
def_fn!(fn17, fn18);
def_fn!(fn18, fn19);
def_fn!(fn19, fn10);
def_fn!(fn20, noop);
def_fn!(fn21, fn22);
def_fn!(fn22, fn23);
def_fn!(fn23, fn24);
def_fn!(fn24, fn25);
def_fn!(fn25, noop);
