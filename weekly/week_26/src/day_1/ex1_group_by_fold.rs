pub fn group_by_fold() {
    let items: [u8; 6] = [1, 1, 2, 3, 3, 3];

    let init_acc = (Vec::<Vec<u8>>::new(), None);

    let (mut grouped, last_item) = items.iter().fold(init_acc, |mut acc, item| {
        // on regarde ou en est l accumulateur
        match acc.1 {
            // vide -> initial state, on le rempli avec l element et le nombre d iteration de ce dernier
            None => acc.1 = Some((item, 1)),
            // rempli, on decompose en (value, count)
            Some((value, mut count)) => {
                //si value == item, on rajoute 1 au count
                if value == item {
                    acc.1 = Some((item, count + 1));
                //sinon on cree le vecteur du groupe, on rempli le vec final et on reinitialise le vec d accumulation avec la nouvelle valeur et un conteur a 1
                } else {
                    let group: Vec<u8> = vec![*value; count];
                    acc.0.push(group);
                    acc.1 = Some((item, 1));
                }
            }
        }
        acc
    });

    //derniere valeur
    if let Some(value) = last_item {
        let group = vec![*value.0; value.1];
        grouped.push(group);
    }

    println!("{:?}", grouped);
}
