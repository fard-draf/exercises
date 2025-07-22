# Audit des differents ypes

## pub struct Id(u64);


Type: wrapper u64,
Taille: 8 bytes,
Alignement : 8 bytes,
Analyse du layout : 
Ce type, bien que wrappe dans une struct, dispose de la representation binaire d'un u64.
Les 8 bytes sont contigues, le premier bytes place sur une adresse multiple de 8.


## Box<u8>

Type: wrapper u8
Taille: 8 bytes,
Alignement: 8 bytes,
Il s'agit d'une référence a une valeur dynamique, donc d'une adresse. Celle ci, sur un x64, est representee par 8 bytes.


## Option<Box<u8>>

Type: wrapper u8
Taille: 8 bytes
Alignement: 8 bytes
Le wrapper Option sur une reference permet d utiliser la niche suivante: une reference est intrinsequement liee a une valeur memoire valide. Il est impossible qu elle soit egale a l adresse memoire 0. 
Ainsi, le wrapper Option utilise l adresse memoire 0 pour y nicher son variant None, qui equivaut a l absence de valeur. 
Le variant Some() est simplement l'adresse memoire valide de la reference, qu'elle soit egale a 0 ou non. 

## u64

Type : u64
Taille: 8 bytes
Alignement: 8 bytes
Un u64 a un representation binaire de 8 bytes, disposés de facon contigue. 
Le premier bit doit etre place a une adresse memoire multiple de 8. 

## Option<u64>

Type: wrapper u64
Taille: 16 bytes
Alignement: 8 bytes
Contrairement a une reference, un u64 peut utiliser toutes les representations binaires disponibles, de 0 a =u64::MAX. Ainsi, le wrapper Option n'a pas la possibilite d utiliser l adresse memoire 0 pour son variant None.
Il doit alors ajouter un tag en tete de du byte du u64, ce qui avec le padding fait un total de 8 bytes (1tag+7padding) + 8 bytes (u64).



