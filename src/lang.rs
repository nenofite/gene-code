//
// The stack-based programming language
//

// A builtin command to run on the stack
#[derive(Clone, Copy)]
pub enum Command {
    Add,
    Sub,
    Mult,
    Div,
    Dup,
}

// Either a piece of data or a command. Programs are sequences of Progs
pub enum Prog {
    D(i32),
    C(Command),
}

// A stack to run programs on, and all other state used by the interpreter
pub struct Stack {
    // The data on the stack (no commands)
    data: Vec<i32>,
    // The stack of commands yet to be executed
    commands: Vec<Command>,
}

impl Stack {
    // Create a new, empty stack
    fn new() -> Stack {
        Stack { data: Vec::new(), commands: Vec::new() }
    }

    // Push data onto the stack
    fn push(&mut self, d: i32) {
        self.data.push(d);
    }

    // Pop data off the stack, or get the default value from an empty stack
    fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(0)
    }

    // Run a single command
    fn run(&mut self, c: Command) {
        use self::Command::*;
        match c {
            Add | Sub | Mult | Div => {
                // Pop two
                let b = self.pop();
                let a = self.pop();
                // Push the result
                self.push(match c {
                    Add => a + b,
                    Sub => a - b,
                    Mult => a * b,
                    Div => a / b,
                    _ => panic!(),
                });
            }
            Dup => {
                // Pop one
                let a = self.pop();
                // Push it twice
                self.push(a);
                self.push(a);
            }
        }
    }

    // Run the given program
    fn run_program(&mut self, program: &[Prog]) {
        // Iterate over each prog
        for p in program {
            match p {
                // If it's data, push it
                &Prog::D(d) => self.push(d),

                // If it's a command, run it
                &Prog::C(c) => self.run(c),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_stack() {
        let mut s = Stack::new();
        // We can push and pop to the stack
        s.push(3);
        s.push(-10);
        assert_eq!(s.pop(), -10);
        assert_eq!(s.pop(), 3);
        assert_eq!(s.data.len(), 0);

        // When popping an empty stack, we get a default value of 0 (not an error)
        assert_eq!(s.pop(), 0);
    }

    #[test]
    fn use_commands() {
        let mut s = Stack::new();

        // We can run builtin commands on the stack
        s.push(3);
        s.push(7);
        s.run(Command::Add); // 3 + 7
        assert_eq!(s.pop(), 10);
        assert_eq!(s.data.len(), 0);

        s.push(7);
        s.run(Command::Sub); // 0 - 7
        assert_eq!(s.pop(), -7);
        assert_eq!(s.data.len(), 0);

        // We can run whole programs (sequences of commands)
        let prog = [Prog::D(10), Prog::D(2), Prog::C(Command::Div), Prog::C(Command::Dup)];
        s.run_program(&prog);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.data.len(), 0);

        // We can run segments of programs, then resume then later
    }
}
