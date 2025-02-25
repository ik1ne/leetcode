struct Solution;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

const OPS: [Op; 4] = [Op::Mul, Op::Div, Op::Add, Op::Sub];

impl Solution {
    pub fn clumsy(mut n: i32) -> i32 {
        let mut stack = vec![n];
        let mut ops = OPS.iter().cycle();

        while n > 1 {
            n -= 1;
            let op = ops.next().unwrap();

            match op {
                Op::Mul => {
                    let top = stack.pop().unwrap();
                    stack.push(top * n);
                }
                Op::Div => {
                    let top = stack.pop().unwrap();
                    stack.push(top / n);
                }
                Op::Add => {
                    stack.push(n);
                }
                Op::Sub => {
                    stack.push(-n);
                }
            }
        }

        stack.iter().sum()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::clumsy(4), 7);
    assert_eq!(Solution::clumsy(10), 12);
}
