// 🎯 Objectif: Maîtriser try_fold pour early termination
// 4 exercices, 15min chacun

// 1. Find first negative (10min)
fn find_first_negative(nums: &[i32]) -> Option<i32> {
    // try_fold avec early return
    match nums
        .iter()
        .try_fold((), |_, &nbr| if nbr < 0 { Err(nbr) } else { Ok(()) })
    {
        Ok(()) => None,
        Err(e) => Some(e),
    }
}
// 10 min -> decouverte de try_fold

// 2. Validate all positive (10min)
fn all_positive(nums: &[i32]) -> bool {
    // try_fold returning Result ou Option
    let result = nums
        .iter()
        .try_fold((), |_, &nbr| if nbr > 0 { Some(()) } else { None });
    result.is_some()
}
//2min
// 3. Parse until error (15min)
fn parse_until_error(strings: &[&str]) -> (Vec<i32>, Option<usize>) {
    // Retourne (parsed_numbers, error_index)
    // try_fold accumulating valid parses

    strings.iter().fold((Vec::new(), None), |mut acc, &words| {
        let parsed = words.parse::<i32>();
        match parsed {
            Ok(value) => {
                acc.0.push(value);
                acc
            }
            Err(_) => {
                if let Some(value) = acc.1 {
                    acc.1 = Some(value + 1);
                    acc
                } else {
                    acc.1 = Some(1);
                    acc
                }
            }
        }
    })
}

//pourquoi try_fold ? je ne vois pas l interet
//7 min

// 4. Sum until limit (15min)
fn sum_until_limit(nums: &[i32], limit: i32) -> Option<(i32, usize)> {
    // Retourne (sum, count) où sum < limit
    // Stop dès que sum >= limit
    nums.iter().try_fold((0, 0_usize), |mut acc, nbr| {
        if acc.0 > limit {
            None
        } else {
            acc.1 += 1;
            acc.0 += nbr;
            Some(acc)
        }
    })

    // pareil le retour n est pas bon, ca devrait etre un option pas un tupe (i32, usize)
    // arret car mauvaise logique de l exercice
    // 5 min
}
