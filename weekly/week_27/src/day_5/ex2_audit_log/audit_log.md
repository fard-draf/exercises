# Audit Library Log

## enum Level 
log/src/lib.rs - line 484

```rust
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Level {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}
```

### Contexte : 

-systeme 64 bits

### Observations:

- attribut => #[repr(usize)] : le comportement memoire de la struct sera similaire a celui d'un usize.
La struct aura donc une representation memoire de :
    -align_of: 8 bytes 
    -size_of: 8 bytes
L'enum peut alors facilement etre transposee en usize a laide de std::mem:transmute().

Sans cet attribut, sa taille par defaut aurait ete optimisee par le LLVM a 1 byte et la correspondance numerique de chaque variant n'aurait pas ete garantie.

## struct Metadata
log/src/lib.rs - line 1082

```rust
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Metadata<'a> {
    level: Level,
    target: &'a str,
}
```

### Observations:

- pas d'attribut memoire - #[repr(Rust)] par defaut
- le champs 'level' est une encapsulation de l'enum du meme nom. -> taille 8 bytes / align 8 bytes

### Analyse :
 - level : 
    - size_of: 8 bytes,
    - align_of: 8 bytes,

 - target : 
    - align_of: 8 bytes,
    - size_of: 16 bytes (fatpointer)

Size_of -> 24 bytes,
Align_of -> 8 bytes,

Le field target presente une reference a une string slice donc il y a une addresse contenant le debut de la string slice et une primitif usize qui contient la longueur de cette derniere.


## struct Record 
log/src/lib.rs - line 766

```rust
#[derive(Clone, Debug)]
pub struct Record<'a> {
    metadata: Metadata<'a>,
    args: fmt::Arguments<'a>,
    module_path: Option<MaybeStaticStr<'a>>,
    file: Option<MaybeStaticStr<'a>>,
    line: Option<u32>,
    #[cfg(feature = "kv")]
    key_values: KeyValues<'a>,
}
```

### Observations:

 - Metadata : 
    -al: 8 bytes,
    -size: 24 bytes,
 - Arguments: 
    -al: 8 bytes,
    -size: 
        16 (fat pointer) + 
        16 (Some(fatpointer)) + 
        16 (Some(fatpointer)) = 48 bytes

 - MaybeStaticStr : 
    -al: 8 bytes,
    -size: 24 (16 bytes + 1 tag + 7 padding) bytes


 - KeyValue : &dyn Source -> 2 * 8 bytes: 16 bytes

 Donc: 
    -metadata: 24,
        (enum / deux varants, le plus grand = 16 (fat pointer), on ajoute le tag et le padding -> 24 bytes )
    -args: 48,
        (struct / 3 fields, 1 fat pointer (16 bytes) + Some(fatpointer) (16 bytes) + 1 fat pointer (16 bytes) -> 48 bytes )
    -module_path: 24,
        ([MaybeStaticStr -> enum / 2 var fat pointer -> 16 bytes + 1 tag + 7 padding -> 24 bytes], Some[MaybeStaticStr] est une opti de niche donc pas d ajout de tag)
    -file: 24,
        (idem que module path -> 24 bytes)
    -line: 8,
        (Some(u32), donc 4 bytes pour la value + 1 pour le tag + 3 pour le padding -> 8 bytes)
    -key_valye: 16,
        (struct / &dyn -> 2*8 bytes -> 16 bytes)


Le retour de size_of et align_of est de (128, 8).
Ma prediction etait de (144,8); J ai 16 bytes de difference, peut etre une optimisation du LLVM. 