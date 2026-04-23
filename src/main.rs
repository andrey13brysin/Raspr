use std::env;
use std::io;
use rand::Rng;
use std::time::Duration;

const MAX_N: u64 = 1_000_000_000_000;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n.is_multiple_of(2) {
        return false;
    }

    let mut d = 3_u64;
    while d <= n / d {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }

    true
}

fn read_number() -> u64 {
    let mut input = String::new();

    loop {
        input.clear();
        println!("Введите целое число n (0..={MAX_N}):");

        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("Ошибка ввода: {err}. Повторите попытку.");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            println!("Пустой ввод. Введите число.");
            continue;
        }

        let parsed = match trimmed.parse::<u64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Некорректный формат. Нужны только неотрицательные целые числа.");
                continue;
            }
        };

        if parsed > MAX_N {
            println!("Слишком большое число. Допустимый диапазон: 0..={MAX_N}.");
            continue;
        }

        return parsed;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::rng();
    loop
    {    
        let n: u64 = if args.len() != 1 {
            args[1].parse().unwrap()
        } else {
            rng.gen_range(0..=MAX_N)
            //read_number()
        };

        if is_prime(n) {
            println!("{n} — простое число");
        } else {
            println!("{n} — не простое число");
        }

        std::thread::sleep(Duration::from_secs(3));
    }
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn prime_check_works() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(13));
        assert!(!is_prime(49));
        assert!(is_prime(1_000_000_007));
    }
}
