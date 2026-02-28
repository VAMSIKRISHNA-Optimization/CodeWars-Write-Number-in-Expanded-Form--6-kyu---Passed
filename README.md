# CodeWars-Write-Number-in-Expanded-Form--6-kyu---Passed
Write Number in Expanded Form
You will be given a number and you will need to return it as a string in Expanded Form. For example:

   12 --> "10 + 2"
   45 --> "40 + 5"
70304 --> "70000 + 300 + 4"
NOTE: All numbers will be whole numbers greater than 0.

TEST CASES:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(expanded_form(2), "2");
        assert_eq!(expanded_form(12), "10 + 2");
        assert_eq!(expanded_form(42), "40 + 2");
        assert_eq!(expanded_form(4982342), "4000000 + 900000 + 80000 + 2000 + 300 + 40 + 2");
    }
    
    #[test]
    fn edge_case_zeros() {
        assert_eq!(expanded_form(420370022), "400000000 + 20000000 + 300000 + 70000 + 20 + 2");
        assert_eq!(expanded_form(70304), "70000 + 300 + 4");
        assert_eq!(expanded_form(9000000), "9000000");
    }
    
    #[test]
    fn edge_case_big_numbers() {
        assert_eq!(expanded_form(92093403034573), "90000000000000 + 2000000000000 + 90000000000 + 3000000000 + 400000000 + 3000000 + 30000 + 4000 + 500 + 70 + 3");
        assert_eq!(expanded_form(2096039485293), "2000000000000 + 90000000000 + 6000000000 + 30000000 + 9000000 + 400000 + 80000 + 5000 + 200 + 90 + 3");
    }
    
    use rand::{thread_rng, Rng};
    
    #[test]
    fn random() {
        let mut rng = thread_rng();
        
        for _ in 0..100 {
            let n = rng.gen_range(1..=100_000_000);
            
            assert_eq!(expanded_form(n), expanded_form_solution(n));
        }
    }
    
    fn expanded_form_solution(n: u64) -> String {
        let s = n.to_string();
    
        s.bytes().enumerate()
            .filter_map(|(i, b)| match b - b'0' {
                0 => None,
                n => Some((n as u64 * 10u64.pow((s.len() - 1 - i) as u32)).to_string()),
            })
            .collect::<Vec<_>>()
            .join(" + ")
    }
}
