use std::io;

enum Operation {
    Plus,
    Minus,
    Divide,
    Multiply
}

struct Calculator {
    prev_result: f64, //string involves lifetimes
    operation: Option<Operation>,
    current_result: f64
}

impl Calculator {
    fn digit(&mut self, dig: f64) {
        match self.operation {
            None => self.prev_result = dig,
            Some(ref x) => self.current_result = dig, //idk what ref does
        }
    }

    fn operate(&mut self, op: Operation) {
        self.operation = Some(op);

    }

    fn equals(&mut self) {
        self.current_result = match self.operation {
            Some(Operation::Plus) => self.prev_result + self.current_result,
            Some(Operation::Minus) => self.prev_result - self.current_result,
            Some(Operation::Divide) => self.prev_result / self.current_result,
            Some(Operation::Multiply) => self.prev_result * self.current_result,
            _ => 0.0
        };

        self.operation = None;
    }
}


fn main() {

    loop {

        let mut calc = Calculator {prev_result: 0.0, operation: None, current_result: 0.0};

        println!("Enter first number");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let num: f64 = input.trim().parse().expect("Please type a number for first number!");
        calc.digit(num);


        println!("Enter Operation");
        input.clear(); //apparently read_line just appends to input instead of overwriting
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().as_ref() {
            "+" => calc.operate(Operation::Plus),
            "-" => calc.operate(Operation::Minus),
            "*" => calc.operate(Operation::Multiply),
            "/" => calc.operate(Operation::Divide),
            _ => println!("error parsing operator, found:{}", input)
        }

        println!("Enter Second Number");
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let num2: f64 = input.trim().parse().expect("Please type a number for second numbe!");
        calc.digit(num2);


        calc.equals();
        println!("Result is: {}", calc.current_result);
    }
}
