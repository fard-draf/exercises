fn main() {}

fn lifetime_colision<'a, 'b>(input1: &'a str, input2: &'b str) -> &'a str
where
    'a: 'b;
{
    if input1.contains("a") {
        input1
    } else {
        input2
    }
}

//  v1. Elision Rules
// 	1. Si une fonction a une ou plusieurs references en parametres, chaque parametre se voit attribuer une lifetime. 
// 	2. Si une fonction n'a qu'un seul parametre avec une reference, et si celle ci retourne une reference en sortie, la lifetime de l'output sera celle de l'input.
// 	3. Si une fonction contient un champ &self ou &mut self et qu'elle retourne une reference, la lifetime de l'output sera liée à la lifetime de la struct.
// 	4. Illustrations: 
// 	```rust

// //les lifetimes sont inferees par rustc. Elles sont annotees ici a titre pedagogique

// fn elision_rule_1<'a, 'b>(input1: &'a String, input2: &'b String) -> (&'a String, &'b String) {
//     (input1, input2)
// }

// fn elision_rule_2<'a>(input1: &'a String, input2: String) -> (&'a String, String) {
//     (input1, input2)
// }

// struct Elision(String);

// impl Elision {
//     fn elision_rule_3<'a, 'b>(&'a self, input2: &'b String) -> &'a String {
//         input2;
//         &self.0
//     }
// }

// ```

// 2.

// ```rust
// fn lifetime_colision<'a, 'b>(input1: &'a str, input2: &'b str) -> &'a str {
//     if input1.contains("a") {
//         input1
//     } else {
//         input2
//     }
// }

// ```

// ```rust
//    Compiling ex2_lifetimes v0.1.0 (/mnt/repo/dev/warehouse/playground/weekly/week_31/day_2/ex2_lifetimes)
// error: lifetime may not live long enough
//  --> day_2/ex2_lifetimes/src/main.rs:7:9
//   |
// 3 | fn lifetime_colision<'a, 'b>(input1: &'a str, input2: &'b str) -> &'a str {
//   |                      --  -- lifetime `'b` defined here
//   |                      |
//   |                      lifetime `'a` defined here
// ...
// 7 |         input2
//   |         ^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
//   |
//   = help: consider adding the following bound: `'b: 'a`

// error: could not compile `ex2_lifetimes` (bin "ex2_lifetimes") due to 1 previous error
// ```

// Ici, Rustc ne peut pas inferer quelle lifetime sera en sortie. Avec un type de sortie determiné à partir d'une condition, la sortie possible est double, la lifetime annotee a est donc invalide dans 1 cas sur 2.


// Ainsi, les deux sorties possibles doivent partager la meme lifetime. 
// ```rust
// fn lifetime_colision<'a, 'b>(input1: &'a str, input2: &'b str) -> &'a str

// {
//     if input1.contains("a") {
//         input1
//     } else {
//         input2
//     }
// }

// ```

// Ou alors, on doit clairement specifier que `'a: 'b` -> `'a` vit au moins aussi longtemps que `'b`. 
// ```rust
// fn lifetime_colision<'a, 'b>(input1: &'a str, input2: &'b str) -> &'a str
// where
//     'a: 'b;
// {
//     if input1.contains("a") {
//         input1
//     } else {
//         input2
//     }
// }

// ```

