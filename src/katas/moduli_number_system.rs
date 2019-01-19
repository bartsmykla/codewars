fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}

fn are_pairwise_coprime(numbers: &Vec<i64>) -> bool {
    for (ni, &n) in numbers.iter().enumerate() {
        for (nk, &k) in numbers.iter().enumerate() {
            if ni == nk {
                continue
            }

            if gcd(n, k) > 1 {
                return false
            }
        }
    }

    true
}

fn from_nb_2str(n: i64, sys: Vec<i64>) -> String {
    let product: i64 = sys.iter().product();

    if product < n || !are_pairwise_coprime(&sys) {
        return String::from("Not applicable");
    }

    sys.iter().map(|k| format!("-{}-", n % k)).collect::<String>()
}

fn testing(n: i64, sys: Vec<i64>, exp: &str) -> () {
    assert_eq!(&from_nb_2str(n, sys), exp)
}

#[test]
fn basics_from_nb_2str() {
    testing(779, vec![8,7,5,3], "-3--2--4--2-");
    testing(187, vec![8,7,5,3], "-3--5--2--1-");
    testing(3450, vec![13,11,7,5,3,2], "-5--7--6--0--0--0-");
}