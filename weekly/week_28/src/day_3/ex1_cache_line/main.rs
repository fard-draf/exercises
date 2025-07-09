fn main() {
    // Le CPU ne lit jamais la mémoire octet par octet.
    // Il la lit par "lignes de cache" (cache lines).
    // Sur la plupart des architectures modernes (x86-64), une ligne de cache fait 64 octets.

    // --- Expérience de pensée 1 : Accès groupé ---

    #[repr(C)]
    struct CacheFriendly {
        a: u64, // 8 octets
        b: u64, // 8 octets
        c: u64, // 8 octets
        d: u64, // 8 octets
        e: u64, // 8 octets
        f: u64, // 8 octets
        g: u64, // 8 octets
        h: u64, // 8 octets
    }
    // Taille totale = 64 octets.

    // Question 1.1 :
    // Quand le CPU a besoin de lire `data.a` pour la première fois, que se passe-t-il VRAIMENT ?
    // Combien d'octets sont chargés depuis la RAM vers le cache L1 ?
    // Commentaire d'explication :
    // 1. Miss L1:                  Le coeur CPU demande une data. Elle n'est pas dans le cache L1, c'est un cache miss. Le processeur est donc en attente.
    // 2. Requete L2:               Le controleur de cache envoie une requete au coeur L2 pour trouver la ligne de 64B contenant la donnee.
    // 3. Miss L2 -> Requete L3:    Si la ligne n'est pas non plus dans le cache L2, le controleur de cache effectue une requete au coeur L3 pour y trouver la donnee.
    // 4. Miss L3 -> Requete RAM:   La ligne n'est toujours pas dans le cache L3, il faut aller la chercher dans la memoire vive. Le processeur envoi une requete a la memoire vive via le controleur memoire. C'est l etape la plus lente.
    // 5. Transfert de la ligne:    La RAM envoie la ligne de cache complete de 64B au processeur (si l'alignement est respecte, un seul trajet, sinon 2).
    // 6. Remplissage:              La ligne de cache remonte toute la hierarchie et y est copiee dans le cache L3, puis L2, puis L1.
    // 7. Hit L1:                   Le coeur CPU redemande la data, c'est un cache hit, il peut reprendre son travail.

    // Question 1.2 :
    // Immédiatement après avoir lu `data.a`, le programme a besoin de `data.b`, puis `data.h`.
    // Quel est le coût de ces accès suivants, comparé au premier ? Pourquoi ?
    // Commentaire d'explication :
    // 1. Initialisation du cache:  La data.a, de 8B, a ete chargee dans le cache L3, L2, L1, apres le cache miss initial. Cette data.a appartient a une struct CacheFriendly, qui est de 64B.
    // 2. Cache hit:                Un autre field de la struct est demande par le coeur du processeur. Comme la struct est de 64B, tous ses fields seront charges en meme temps (on part du principe que l'alignement est respecte). C est donc un cache hit.
    // 3. Cout:                     Le cout de l'acces a ce field et aux fields suivant est quasi nulle ( 4 a 5 cylces) car il n'y aura pas de cache miss, la struct est entierement chargee sur le cache car <= 64B.

    // --- Expérience de pensée 2 : Accès dispersé ---

    #[repr(C)]
    struct CacheHostile {
        a: u64,
        _padding1: [u8; 56], // On "saute" le reste de la première ligne de cache
        b: u64,
        _padding2: [u8; 56], // On "saute" le reste de la deuxième ligne de cache
    }
    // Taille totale = 64 + 64 = 128 octets.

    // Question 2.1 :
    // Le CPU a besoin de lire `hostile_data.a`. Que se passe-t-il ?
    // (La réponse est similaire à 1.1)
    // Commentaire d'explication :
    // 1. Miss L1:                  Le coeur du CPU demande la data hostile_data.a. Elle n'est pas dans le cache L1. C'est un cache miss, le processeur est mit en attente.
    // 2. Requete L2:               Le controleur de cache envoie une requete au cache L2 pour voir si la data s'y trouve. Encore une fois, c'est un cache miss.
    // 3. Requete L3:               Le controleur de cache envoie une requete au cache L3 pour y trouver la data demandee. Encore une fois, c'est un cache miss.
    // 4. Requete RAM:              La data n'est pas dans les caches CPU -> le processeur envoie une requete via le controleur de memoire a la memoire vive, contenant l'adresse de la donne requise.
    // 5. Transfert de la ligne:    La RAM cherche la donnee a l adresse specifiee: Il prend les 64B sur laquelle la donnee est alignee. Comme celle ci fait partie d une struct de 128B, seul la moitiee de la struct sera transmise.
    // 6. Remplissage:              La cache line est copiee sur la L3, L2, L1.
    // 7. Hit L1:                   Le coeur du processeur demande acces a la data hostile_data.a -> c'est un cache hit, le processeur reprend son travail.

    // Question 2.2 :
    // Immédiatement après, le programme a besoin de `hostile_data.b`.
    // Décrivez la séquence d'événements au niveau du cache.
    // Pourquoi est-ce radicalement plus lent que dans le cas "CacheFriendly" ?
    // Utilisez le terme "cache miss".
    // Commentaire d'explication :
    // Des lors qu'une struct est > 64B, elle sera fragmentee en plusieurs partie. En effet, les limitations materielles actuelles ne permettent de transporter des caches lines de 64B maximum (sur des x64).
    // Ainsi, on peut voir ici que l offset de hostile_data.b est > 64B, il n'est pas present dans la premiere ligne de cache chargee qui avait pour offset 0.
    // C'est donc un cache miss, le processur de requete RAM pour arriver au cache hit doit recommencer. C'est donc beaucoup plus lent que d'avoir acces instantanemment a la data sur le cache cpu.

    // --- Synthèse ---
    // Question 3 :
    // Rédigez en une phrase l'ancrage du jour.
    // Mon code ne parle pas au CPU, il parle au ...
    // ...gestionnaire de cache ! On n'envoi pas le cuistot faire les courses quand il est en train de faire son service!
}
