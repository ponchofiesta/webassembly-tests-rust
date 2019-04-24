
pub fn prime(max: usize) -> Vec<usize> {

    let mut primes: Vec<usize> = vec![];

    if max > 2 {
        primes.push(2);
    }
    if max > 3 {
        primes.push(3);
    }

    let mut sieve = vec![false; max];

    for x in 1..max {
        if x * x >= max {
            break;
        }
        for y in 1..max {
            if y * y >= max {
                break;
            }

            // Main part of Sieve of Atkin
            let mut n = (4 * x * x) + (y * y);
            if n <= max && (n % 12 == 1 || n % 12 == 5) {
                sieve[n] ^= true;
            }

            n = (3 * x * x) + (y * y);
            if n <= max && n % 12 == 7 {
                sieve[n] ^= true;
            }

            n = (3 * x * x).checked_sub(y * y).unwrap_or(0);
            if x > y && n <= max && n % 12 == 11 {
                sieve[n] ^= true;
            }
        }
    }

    // Mark all multiples of squares as non-prime
    for r in 5..max {
        if r * r >= max {
            break;
        }
        if sieve[r] {
            let mut i = r * r;
            while i < max {
                sieve[i] = false;
                i += r * r;
            }
        }
    }

    // Print primes using sieve[]
    for a in 5..max {
        if sieve[a] {
            primes.push(a);
        }
    }

    primes
}
