# Ma "Cheat Sheet" Mémoire Rust

## 1. Principes Fondamentaux

* **`size_of<T>()` :** 
* **`align_of<T>()` :** 
* **Padding :** 

## 2. Le Jeu du Tetris Mémoire (Padding en pratique)

* **Struct NON optimisée :**
    ```rust
    struct Bad {
        a: u8,  // 1 octet + 7 de padding
        b: u64, // 8 octets
        c: u8,  // 1 octet + 7 de padding
    }
    // size = 24, align = 8
    ```
* **Struct optimisée :**
    ```rust
    struct Good {
        b: u64, // 8 octets
        a: u8,  // 1 octet
        c: u8,  // 1 octet + 6 de padding
    }
    // size = 16, align = 8
    ```

## 3. Optimisation de Niche (`Option<T>`)

* **Le Principe :** Si un type `T` a une valeur binaire "invalide" (comme un pointeur nul), `Option<T>` utilise cette valeur pour représenter `None`, évitant ainsi un surcoût en mémoire.
* **Cas qui fonctionnent :**
    * `Option<Box<T>>`
    * `Option<&T>`
    * `Option<NonNull<T>>`
* **La Leçon de la Crate `log` :**
    * L'optimisation peut aussi s'appliquer à des `enum` personnalisés (`Option<MaybeStaticStr>`) si le compilateur peut prouver que toutes les variantes contiennent un pointeur non-nul. C'est une optimisation avancée et puissante.

## 4. Compilation Conditionnelle (`#[cfg]`)

* **Le Principe :** L'attribut `#[cfg(feature = "...")]` inclut ou exclut un champ d'une structure au moment de la compilation.
* **Impact :** La taille et le layout d'une même `struct` peuvent varier radicalement en fonction des *features* activées. **Toujours vérifier le contexte de compilation.**