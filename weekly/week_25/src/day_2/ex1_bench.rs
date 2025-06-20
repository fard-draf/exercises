pub fn sum_manual_loop(data: &[u64]) -> u64 {
    let mut sum = 0;
    for &item in data {
        sum += item;
    }
    sum
}

pub fn sum_iterator_sum(data: &[u64]) -> u64 {
    data.iter().sum()
}

pub fn sum_iterator_fold(data: &[u64]) -> u64 {
    data.iter().fold(0, |acc, &x| acc + x)
}

// pub fn vec_by_fold (data: &[&str]) -> Vec<String> {
//     data.iter().fold(Vec::new(), |mut acc, &slice| {

//         acc.push(slice.trim().to_lowercase());

//         acc

//     })
// }

// pub fn vec_by_loop(data: &[&str]) -> Vec<String> {
//     let mut vec = Vec::new();
//     for &slice in data {
//         vec.push(slice.trim().to_lowercase());
//     }
//     vec
// }

// pub fn vec_by_loop_with_capacity(data: &[&str]) -> Vec<String> {
//     // On réserve la mémoire EXACTEMENT nécessaire en une seule fois
//     let mut vec = Vec::with_capacity(data.len());

//     for &slice in data {
//         // Chaque push est maintenant "gratuit" (pas de risque de réallocation)
//         vec.push(slice.trim().to_lowercase());
//     }
//     vec
// }

// pub fn vec_by_map(data: &[&str]) -> Vec<String> {
//     data.iter()
//         .map(|&slice| slice.trim().to_lowercase())
//         .collect() // Pas besoin de spécifier le type, il est inféré
// }

pub fn stack_vec_by_fold(data: &[u64]) -> Vec<u64> {
    data.iter().fold(Vec::new(), |mut acc, &slice| {
        acc.push(slice);

        acc
    })
}

pub fn stack_vec_by_loop(data: &[u64]) -> Vec<u64> {
    let mut vec = Vec::new();
    for &slice in data {
        vec.push(slice);
    }
    vec
}

pub fn stack_vec_by_loop_with_capacity(data: &[u64]) -> Vec<u64> {
    // On réserve la mémoire EXACTEMENT nécessaire en une seule fois
    let mut vec = Vec::with_capacity(data.len());

    for &slice in data {
        // Chaque push est maintenant "gratuit" (pas de risque de réallocation)
        vec.push(slice);
    }
    vec
}

pub fn stack_vec_by_map(data: &[u64]) -> Vec<u64> {
    data.iter().map(|&slice| slice).collect::<Vec<_>>() // Pas besoin de spécifier le type, il est inféré
}
