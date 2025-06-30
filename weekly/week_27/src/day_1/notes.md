
==========TEST1
creation_stack          time:   [1.0150 ns 1.0156 ns 1.0162 ns]

Found 9 outliers among 100 measurements (9.00%)

  7 (7.00%) high mild

  2 (2.00%) high severe



creation_heap           time:   [14.075 ns 14.101 ns 14.126 ns]

Found 3 outliers among 100 measurements (3.00%)

  2 (2.00%) low mild

  1 (1.00%) high severe



access_stack            time:   [343.77 ps 355.53 ps 377.45 ps]

Found 18 outliers among 100 measurements (18.00%)

  3 (3.00%) high mild

  15 (15.00%) high severe



access_heap             time:   [339.28 ps 339.84 ps 340.62 ps]

Found 15 outliers among 100 measurements (15.00%)

  3 (3.00%) high mild

  12 (12.00%) high severe


==================TEST2

     Running benches/alloc_bench.rs (target/release/deps/alloc_bench-35e1939fde38e77f)
Gnuplot not found, using plotters backend
creation_stack          time:   [1.0139 ns 1.0142 ns 1.0146 ns]
                        change: [−0.1942% −0.1234% −0.0594%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

creation_heap           time:   [13.583 ns 13.599 ns 13.615 ns]
                        change: [−3.6686% −3.4169% −3.1625%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

access_stack            time:   [338.14 ps 338.26 ps 338.38 ps]
                        change: [−5.7161% −2.9467% −1.2241%] (p = 0.01 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe

access_heap             time:   [338.27 ps 338.41 ps 338.55 ps]
                        change: [−2.4764% −1.6459% −0.9306%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

  =======================================

Entre la creation stack et heap, ok 14xplus de temps pr la heap, c est logique, il faut 
du temps d'allocation de chaque Box.

Mais concernant l acces, les deux, stack et heap, ont le meme delai. Pourquoi ? Est ce parce que la heap est en cache sur le stack ? Je ne comprends pas encore clairement le fonctionnement du cache.


Pourquoi creation_stack est rapide et pourquoi creation_heap est lent ?

CS est rapide car il s'agit ici du LIFO, le cpu a un acces instantane a la stack, et doit simplement "entasser" les allocations memoires. Le temps de trajet de l'information est quasi-nul.

En revanche, le CH est plus lent car un `Box<T>` est une allocation dynamique d'une valeur. Le cpu attribue une adresse memoire au pointeur sur la stack puis va allouer l'espace a cet adresse avec la valeur en question. Il doit faire cette manipulation pour chaque donnee, cela prend donc du temps de trajet d'information dans les circuits imprimés du cpu et de la ram. 

Pourquoi access_stack et access_heap ont le meme temps d execution alors ?

Ils n ont absolument pas le meme temps d execution.
Cela est un side_effect du benchmark car la moyenne ecrase la difference.
En effet, la premiere operation est viceralement differente: 
 - AS est sur la stack -> Acces quasi-instantanne. 
 - AH est sur la heap. Le cpu doit lire l adresse memoire, voir que c est une reference, aller a cette reference puis retourner la valeur. Le temps de trajet de l information est beaucoup plus long (on l a vu avec la creation, x14 environ, ce qui est bien sur variable mais c est pour avoir une idee).

 Cependant, pourquoi les tests sont quasi egaux si le temps d execution n est pas = ? 
 Car lors de la premiere operation, le cpu check si son cache contient la valeur a l adresse demandee: pour les deux cas, comme ce sont les cas 0 -> non, le cache cpu contient peut etre une valeur mais ce n est pas celle demandee. Miss cache. 
 
 Alors s engage la manipulation precedement evoquee. 
 Pendant ce cas 0, dans les deux cas de figure, le cpu ajoute en cache cpu la valeur. Si jamais elle est de nouveau demandee, l acces est immediat. 

 Les caches cpu sont relativement "grand" (suivant les couches) donc ils peuvent stocker plusieurs adresses lors de chaque thread (gestion du cache, algo complexes, LRU, etc).

 Donc une fois le benchmark lance, et les cas 0 depasses, le temps d execution entre AS et AH sont egaux -> meme localite du cache -> cpu cache

