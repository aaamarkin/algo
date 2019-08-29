#[macro_export]
macro_rules! myprintln {
      () => ();
      ($($arg:tt)*) => (print!(""));
//    () => ($crate::print!("\n"));
//    ($($arg:tt)*) => (println!($($arg)*));
}
#[macro_export]
macro_rules! myprintln2 {
//      () => ();
//      ($($arg:tt)*) => (print!(""));
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (println!($($arg)*));
}