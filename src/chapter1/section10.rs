// 소수인지 확인하는 함수
fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 소수 100개를 구하는 함수
fn get_primes(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    // count가 100이 될 때까지 반복
    while count < 100 {
        // 소수인 경우 primes에 추가
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
    }

    #[test]
    fn test_get_primes() {
        let mut primes = [0; 100];
        get_primes(&mut primes);
        println!(" primes: {:?}", primes);
        assert_eq!(primes[0], 2);
        assert_eq!(primes[1], 3);
        assert_eq!(primes[2], 5);
        assert_eq!(primes[3], 7);
        assert_eq!(primes[4], 11);
        assert_eq!(primes[5], 13);
        assert_eq!(primes[6], 17);
        assert_eq!(primes[7], 19);
        assert_eq!(primes[8], 23);
        assert_eq!(primes[9], 29);
    }
}
