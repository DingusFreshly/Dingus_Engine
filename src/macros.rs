//trying to learn macros gang
macro_rules! say_hello {
    ($name: expr) => {
        println!("hello {}!", $name);
    }
}

#[macro_export]
macro_rules! say_hello {
    ($name: expr) => {
        println!("hello {}!", $name); 
    };
}

#[macro_export]
macro_rules! print_all {
    (
    $($num: expr);* 
) => {
        $(
            println!("{}", $num);
        )*
    };
}
#[macro_export]
macro_rules! local {
    () => {
        let
    };
}

macro_rules! sum {
    ($($num:expr),* $(,)?) => {
        0 $(+ $num)*
    };
}
macro_rules! print_all {
    ($prefix:expr ; $($name:expr),* $(,)?) => {
        $(
            println!("{}{}", $prefix, $name);
        
        )*
    };
    ($($var:expr),* $(,)?) => {
        println!("{}", $var);
    }
}
#[macro_export]
macro_rules! foo_the_bar {
    ($till_she_what:expr) => {
        println!("foo till she {}", $till_she_what);
    };
    () => {
        println!("foo till she bar")   
    }
}