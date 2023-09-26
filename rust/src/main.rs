#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod core;
mod lang;
mod render;
//mod comp;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::core::*;
use crate::lang::*;
use crate::render::*;
//use crate::comp::*;

fn main() {
  // Initializes the book
  let book = &mut Book::new();

  // Church Nat constructors
  let c_z = define(book, "c_z", "$ (0 * (0 a a))");
  let c_s = define(book, "c_s", "$ (0 (0 s (0 z k)) (0 (1 (0 k r) s) (0 z r)))");

  // Utils
  define(book, "id", "$ (0 x x)");

  // Church Nat constants
  {
    define(book, "c0" , "$ (0 * (0 a a))");
    define(book, "c1" , "$ (0 (0 a R) (0 a R))");
    define(book, "c2" , "$ (0 (1 (0 b a) (0 a R)) (0 b R))");
    define(book, "c3" , "$ (0 (1 (1 (0 c b) (0 b a)) (0 a R)) (0 c R))");
    define(book, "c4" , "$ (0 (1 (1 (1 (0 d c) (0 c b)) (0 b a)) (0 a R)) (0 d R))");
    define(book, "c5" , "$ (0 (1 (1 (1 (1 (0 e d) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 e R))");
    define(book, "c6" , "$ (0 (1 (1 (1 (1 (1 (0 f e) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 f R))");
    define(book, "c7" , "$ (0 (1 (1 (1 (1 (1 (1 (0 g f) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 g R))");
    define(book, "c8" , "$ (0 (1 (1 (1 (1 (1 (1 (1 (0 h g) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 h R))");
    define(book, "c9" , "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (0 i h) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 i R))");
    define(book, "c10", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 j i) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 j R))");
    define(book, "c11", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 k j) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 k R))");
    define(book, "c12", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 l k) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 l R))");
    define(book, "c13", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 m l) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 m R))");
    define(book, "c14", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 n m) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 n R))");
    define(book, "c15", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 o n) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 o R))");
    define(book, "c16", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 p o) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 q R))");
    define(book, "c17", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 q p) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 q R))");
    define(book, "c18", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 r q) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 r R))");
    define(book, "c19", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 s r) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 s R))");
    define(book, "c20", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 t s) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 t R))");
    define(book, "c21", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 u t) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 u R))");
    define(book, "c22", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 v u) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 v R))");
    define(book, "c23", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 w v) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 w R))");
    define(book, "c24", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 x w) (0 w v)) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 x R))");
    define(book, "c25", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 y x) (0 x w)) (0 w v)) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 x R))");
    define(book, "c26", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 z y) (0 y x)) (0 x w)) (0 w v)) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 x R))");
    //define(book, "c26", "$ (0 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (1 (0 z y) (0 y x)) (0 x w)) (0 w v)) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 x R))");

    define(book, "k0" , "$ (0 * (0 a a))");
    define(book, "k1" , "$ (0 (0 a R) (0 a R))");
    define(book, "k2" , "$ (0 (2 (0 b a) (0 a R)) (0 b R))");
    define(book, "k3" , "$ (0 (2 (2 (0 c b) (0 b a)) (0 a R)) (0 c R))");
    define(book, "k4" , "$ (0 (2 (2 (2 (0 d c) (0 c b)) (0 b a)) (0 a R)) (0 d R))");
    define(book, "k5" , "$ (0 (2 (2 (2 (2 (0 e d) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 e R))");
    define(book, "k6" , "$ (0 (2 (2 (2 (2 (2 (0 f e) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 f R))");
    define(book, "k7" , "$ (0 (2 (2 (2 (2 (2 (2 (0 g f) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 g R))");
    define(book, "k8" , "$ (0 (2 (2 (2 (2 (2 (2 (2 (0 h g) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 h R))");
    define(book, "k9" , "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (0 i h) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 i R))");
    define(book, "k10", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 j i) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 j R))");
    define(book, "k11", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 k j) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 k R))");
    define(book, "k12", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 l k) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 l R))");
    define(book, "k13", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 m l) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 m R))");
    define(book, "k14", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 n m) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 n R))");
    define(book, "k15", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 o n) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 o R))");
    define(book, "k16", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 p o) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 p R))");
    define(book, "k17", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 q p) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 q R))");
    define(book, "k18", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 r q) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 r R))");
    define(book, "k19", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 s r) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 s R))");
    define(book, "k20", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 t s) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 t R))");
    define(book, "k21", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 u t) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 u R))");
    define(book, "k22", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 v u) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 v R))");
    define(book, "k23", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 w v) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 w R))");
    define(book, "k24", "$ (0 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (2 (0 x w) (0 w v)) (0 v u)) (0 u t)) (0 t s)) (0 s r)) (0 r q)) (0 q p)) (0 p o)) (0 o n)) (0 n m)) (0 m l)) (0 l k)) (0 k j)) (0 j i)) (0 i h)) (0 h g)) (0 g f)) (0 f e)) (0 e d)) (0 d c)) (0 c b)) (0 b a)) (0 a R)) (0 x R))");
  }

  // Church Nats
  define(book, "mul", "$ (0 (0 a b) (0 (0 c a) (0 c b)))");

  // Bools
  define(book, "T", "$ (0 t (0 * t))");
  define(book, "F", "$ (0 * (0 f f))");
  define(book, "not", "$ (0 (0 f (0 t r)) (0 t (0 f r)))");
  define(book, "and", "$ (0 (0 (0 (0 @T (0 @F a)) a) (0 (0 (0 @F (0 @F b)) b) c)) c)");

  // Scott Nats
  define(book, "S", "$ (0 a (0 (0 a b) (0 * b)))");
  define(book, "Z", "$ (0 * (0 a a))");

  // Generators for a big binary tree
  // λr. λt. ((t r) r)
  //define(book, "g_s", "$ (0 (2 (0 b c) b) c)");
  define(book, "g_s", "$ (0 (2 r0 r1) (0 (0 r0 (0 r1 r)) r))");
  define(book, "g_z", "$ (0 x x)");

  // BitString constructors
  // O = λxs λo λi λe (o xs)
  // I = λxs λo λi λe (i xs)
  // E =     λo λi λe e
  define(book, "O", "$ (0 xs (0 (0 xs r) (0 * (0 * r))))");
  define(book, "I", "$ (0 xs (0 * (0 (0 xs r) (0 * r))))");
  define(book, "E", "$ (0 * (0 * (0 e e)))");

  // Double
  define(book, "nidS", "
    $ (0 p ret)
    & @S   ~ (0 nidp ret)
    & @nid ~ (0 p nidp)
  ");
  define(book, "nid" , "
    $ (0 (0 @nidS (0 @Z ret)) ret)
  ");

  // Decrements a BitString
  // decO = λp(I (dec p))
  // decI = λp(low p)
  // dec  = λx(((x decO) decI) E)
  define(book, "decO", "
    $ (0 p idecp)
    & @I   ~ (0 decp idecp)
    & @dec ~ (0 p decp)
  ");
  define(book, "decI", "
    $ (0 p lowp)
    & @low ~ (0 p lowp)
  ");
  define(book, "dec" , "
    $ (0 (0 @decO (0 @decI (0 @E ret))) ret)
  ");

  // Auxiliary function
  // lowO = λp(O (O p))
  // lowI = λp(O (I p))
  // low  = λx(((x lowO) lowI) E)
  define(book, "lowO", "
    $ (0 p oop)
    & @O ~ (0 p op)
    & @O ~ (0 op oop)
  ");
  define(book, "lowI", "
    $ (0 p oip)
    & @I ~ (0 p ip)
    & @O ~ (0 ip oip)
  ");
  define(book, "low" , "
    $ (0 (0 @lowO (0 @lowI (0 @E ret))) ret)
  ");

  // Decrements a BitString until it is zero
  // runO = λp(run (dec (O p)))
  // runI = λp(run (dec (I p)))
  // run  = λx(((x runO) runI) E)
  define(book, "runO", "
    $ (0 p ret)
    & @run ~ (0 decop ret)
    & @dec ~ (0 op decop)
    & @O   ~ (0 p op)
  ");
  define(book, "runI", "
    $ (0 p ret)
    & @run ~ (0 decip ret)
    & @dec ~ (0 ip decip)
    & @I   ~ (0 p ip)
  ");
  define(book, "run" , "
    $ (0 (0 @runO (0 @runI (0 @E ret))) ret)
  ");

  // Decrements 2^N BitStrings until they reach zero
  // brnZ = (run (c8 S Z))
  // brnS = λp {(brn p) (brn p)}
  // brn  = λn ((n brnS) brnZ)
  define(book, "brnZ", "
    $ ret
    & @run ~ (0 val ret)
    & @mul ~ (0 @c2 (0 @c5 (0 @I (0 @E val))))
  ");
  define(book, "brnS", "
    $ (0 (1 p0 p1) (0 r0 r1))
    & @brn ~ (0 p0 r0)
    & @brn ~ (0 p1 r1)
  ");
  define(book, "brn", "
    $ (0 (0 @brnS (0 @brnZ r)) r)
  ");

  // af  = λx (x afS afZ)
  // afS = λp (and (af p) (af p))
  // afZ = T
  define(book, "af", "
    $ (0 (0 @afS (0 @afZ a)) a)
  ");
  define(book, "afS", "
    $ (0 (1 a b) c)
    & (0 b d) ~ @af
    & (0 e (0 d c)) ~ @and
    & (0 a e) ~ @af
  ");
  define(book, "afZ", "
    $ @T
  ");

  // Church multiplication.
  define(book, "ex0", "
    $ root
    & @c2 ~ (0 @k2 root)
  ");

  // Allocates a big tree.
  define(book, "ex1", "
    $ root
    & @c24 ~ (0 @g_s (0 @g_z root))
  ");

  // Decrease a binary counter.
  define(book, "ex2", "
    $ main
    & @c22 ~ (0 @I (0 @E nie))
    & @run ~ (0 nie main)
  "); 

  // Decreases many binary counters.
  define(book, "ex3", "
    $ res
    & @c12 ~ (0 @S (0 @Z dep))
    & @brn ~ (0 dep res)
  "); 

  // Native Numbers
  // Number literals lazily expand to λ-encoded bitstrings. For example, the number
  // 26 will expand to `λo λi λe (o 13)`, then `λo λi λe (o λo λi λe (i 6))`, etc.

  // Divides a native number by 2
  define(book, "half", "$ (0 (0 (0 op op) (0 (0 ip ip) (0 0 ret))) ret)");

  // (x λp.p λp.p 0)
  define(book, "ex4", "
    $ ret
    & {+ -2 ret}
    ~ +1
  "); 

  define(book, "1", "
    $ (0 a (0 b (0 (0 a (0 b c)) (0 * c))))
  ");
  define(book, "2", "
    $ (0 a (0 * (0 (0 a b) b)))
  ");
  define(book, "3", "
    $ (0 * (0 a a))
  ");
  define(book, "4", "
    $ (0 a (0 (0 a b) (0 * b)))
  ");
  define(book, "5", "
    $ (0 (0 a (0 b c)) (0 b (0 a c)))
  ");
  define(book, "6", "
    $ (0 a (0 * a))
  ");
  define(book, "7", "
    $ (0 * (0 a a))
  ");
  define(book, "8", "
    $ (0 a b)
    & (0 a (0 @6 b))
    ~ @9
  ");
  define(book, "9", "
    $ (0 (0 @A (0 @B a)) a)
  ");
  define(book, "A", "
    $ (0 (1 a b) (0 * c))
    & (0 b (0 @7 d))
    ~ @9
    & (0 e (0 d c))
    ~ @1
    & (0 a (0 @6 e))
    ~ @9
  ");
  define(book, "B", "
    $ (0 a b)
    & (0 a b)
    ~ @2
  ");
  define(book, "C", "
    $ (0 (0 @D (0 @E a)) a)
  ");
  define(book, "D", "
    $ (0 a (0 b (0 (1 c d) e)))
    & (0 b (0 d f))
    ~ @C
    & (0 g (0 f e))
    ~ @1
    & (0 a (0 c g))
    ~ @C
  ");
  define(book, "E", "
    $ (0 a (0 (0 a b) c))
    & (0 b c)
    ~ @2
  ");
  define(book, "F", "
    $ (0 a b)
    & (0 a (0 @5 b))
    ~ @C
  ");
  define(book, "G", "
    $ a
    & (0 @3 a)
    ~ @4
  ");
  define(book, "H", "
    $ a
    & (0 @G a)
    ~ @4
  ");
  define(book, "I", "
    $ a
    & (0 @H a)
    ~ @4
  ");
  define(book, "J", "
    $ a
    & (0 @I a)
    ~ @4
  ");
  define(book, "K", "
    $ a
    & (0 @J a)
    ~ @4
  ");
  define(book, "L", "
    $ a
    & (0 @K a)
    ~ @4
  ");
  define(book, "M", "
    $ a
    & (0 @L a)
    ~ @4
  ");
  define(book, "N", "
    $ a
    & (0 @M a)
    ~ @4
  ");
  define(book, "O", "
    $ a
    & (0 @N a)
    ~ @4
  ");
  define(book, "P", "
    $ a
    & (0 @O a)
    ~ @4
  ");
  define(book, "Q", "
    $ a
    & (0 @P a)
    ~ @4
  ");
  define(book, "R", "
    $ a
    & (0 @Q a)
    ~ @4
  ");
  define(book, "S", "
    $ a
    & (0 @R a)
    ~ @4
  ");
  define(book, "T", "
    $ a
    & (0 @S a)
    ~ @4
  ");
  define(book, "U", "
    $ a
    & (0 @T a)
    ~ @4
  ");
  define(book, "V", "
    $ a
    & (0 @U a)
    ~ @4
  ");
  define(book, "Main", "
    $ a
    & (0 b a)
    ~ @F
    & (0 @V b)
    ~ @8
  "); 
  
  // Initializes the net
  let net = &mut Net::new(1 << 28);
  let mut image = [0; 256 * 256 * 3];
  let (mut event_pump, mut canvas) = render::setup_window();

  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
        _ => (),
      };
    };

    // Marks initial time
    let start = std::time::Instant::now();
    
    net.boot(name_to_val("Main"));
    net.expand(book, Ptr::new(VRR,0));
    net.reduce(book);
    // Computes its normal form
    net.normal(book);
    
    //Shows results and stats
    println!("RWTS: {}", net.rwts);
    println!("DREF: {}", net.dref);
    println!("TIME: {:.3} s", (start.elapsed().as_millis() as f64) / 1000.0);
    println!("RPS : {:.3} million", (net.rwts as f64) / (start.elapsed().as_millis() as f64) / 1000.0);
    
    render::net_bin_tree_to_image(net, &mut image);
    render::render_image(&mut canvas, &mut image);
    
    println!("FPS : {:.3}", (1.0 / (start.elapsed().as_millis() as f64) * 1000.0));
  }
}
