pub struct Evaluator {
    ip: usize,
    tokens: Vec<usize>,
}

impl Evaluator {
    pub fn new(tokens: Vec<usize>) -> Self {
        Evaluator {
            ip: 0,
            tokens,
        }
    }

    pub fn evaluate(&mut self) -> usize {
        while self.ip < self.tokens.len() {
            let operator = self.get_operator();

            if operator == 99 {
//                println!("{:?}", self.tokens);
                return self.get_first();
            }

            let (_, first, second, third) = self.get_operation();
            match operator {
                1 => self.op(Operator::Add, first, second, third),
                2 => self.op(Operator::Multiply, first, second, third),
                _ => panic!()
            }
        }
//        println!("{:?}", self.tokens);
        self.get_first()
    }

    fn op(&mut self, op: Operator, first: usize, second: usize, store: usize) {
        let first = self.get_value_at(first);
        let second = self.get_value_at(second);
        let result = match op {
            Operator::Add => first + second,
            Operator::Multiply => first * second,
        };
        self.tokens.remove(store);
        self.tokens.insert(store, result);
    }

    fn get_operation(&mut self) -> (usize, usize, usize, usize) {
        let operator = self.get_operator();
        let operands = self.get_operands();
        self.ip += 4;
        (operator, operands.0, operands.1, operands.2)
    }

    fn get_operator(&self) -> usize {
        self.get_value_at(self.ip)
    }

    fn get_operands(&self) -> (usize, usize, usize) {
        (
            self.get_value_at(self.ip + 1),
            self.get_value_at(self.ip + 2),
            self.get_value_at(self.ip + 3),
        )
    }

    fn get_value_at(&self, pos: usize) -> usize {
        *self.tokens.get(pos).unwrap()
    }

    fn get_first(&self) -> usize {
        self.get_value_at(0)
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}