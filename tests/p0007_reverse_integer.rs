impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result: i32 = 0;

        while x != 0 {
            let pop = x % 10;
            x /= 10;

            result = match result.checked_mul(10) {
                None => return 0,
                Some(v) => v,
            };

            result = match result.checked_add(pop) {
                None => return 0,
                Some(v) => v,
            };
        }

        result
    }
}

struct Solution;

#[test]
fn case1() {
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn case2() {
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn case3() {
    assert_eq!(Solution::reverse(120), 21);
}
