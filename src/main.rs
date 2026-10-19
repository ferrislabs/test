use std::env;

/// Calcule F(n) par programmation dynamique (itératif, O(n) temps, O(1) mémoire).
/// Retourne `None` en cas de dépassement de `u128`.
fn fibonacci(n: u32) -> Option<u128> {
    let (mut prev, mut curr) = (0u128, 1u128);
    if n == 0 {
        return Some(0);
    }
    for _ in 1..n {
        let next = prev.checked_add(curr)?;
        prev = curr;
        curr = next;
    }
    Some(curr)
}

/// Retourne la table F(0..=n) construite par programmation dynamique.
fn fibonacci_table(n: usize) -> Vec<u128> {
    let mut dp = vec![0u128; n + 1];
    if n >= 1 {
        dp[1] = 1;
    }
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp
}

fn main() {
    let n: u32 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    match fibonacci(n) {
        Some(v) => println!("F({n}) = {v}"),
        None => eprintln!("F({n}) dépasse la capacité d'un u128 (n max = 186)"),
    }
    if n <= 186 {
        let t: Vec<String> = fibonacci_table(n as usize).iter().map(u128::to_string).collect();
        println!("{}", t.join(" "));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_values() {
        let expected = [0u128, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (i, e) in expected.iter().enumerate() {
            assert_eq!(fibonacci(i as u32), Some(*e));
        }
        assert_eq!(fibonacci_table(10), expected);
    }

    #[test]
    fn limits() {
        assert!(fibonacci(186).is_some());
        assert_eq!(fibonacci(187), None);
    }
}
