//
// The stack-based programming language
//

use std::fmt::Debug;

// A builtin command to run on the stack
#[derive(Clone, Copy, Debug)]
pub enum Command {
    Add,
    Sub,
    Mult,
    Div,
    Dup,
    Swap,
}

// Either a piece of data or a command. Programs are sequences of Progs
#[derive(Clone, Copy, Debug)]
pub enum Prog {
    D(i32),
    C(Command),
}

// A stack to run programs on, and all other state used by the interpreter
#[derive(Clone, Debug)]
pub struct Stack {
    // The data on the stack (no commands)
    data: Vec<i32>,
    // The stack of commands yet to be executed
    commands: Vec<Prog>,
}

impl Stack {
    // Create a new, empty stack
    pub fn new() -> Stack {
        Stack { data: Vec::new(), commands: Vec::new() }
    }

    // Push data onto the stack
    pub fn push(&mut self, d: i32) {
        self.data.push(d);
    }

    // Pop data off the stack, or get the default value from an empty stack
    pub fn pop(&mut self) -> i32 {
        self.data.pop().unwrap_or(0)
    }

    // Run a single command
    pub fn run(&mut self, c: Command) {
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
                    Div => if b != 0 { a / b } else { 0 },
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
            Swap => {
                // Pop two
                let b = self.pop();
                let a = self.pop();
                // Push reverse
                self.push(b);
                self.push(a);
            }
        }
    }

    // Queue the given program into the command stack. This doesn't actually run anything.
    pub fn queue_program(&mut self, program: &[Prog]) {
        // Copy the program into the top of the stack
        for p in program.iter().rev() {
            self.commands.push(*p);
        }
    }

    // Run the next command on the stack. Does nothing if the stack is empty.
    pub fn run_next(&mut self) {
        if let Some(p) = self.commands.pop() {
            match p {
                Prog::D(d) => self.push(d),
                Prog::C(c) => self.run(c),
            }
        }
    }

    // Run until the command stack is empty. Returns the number of steps taken.
    pub fn run_all(&mut self) -> usize {
        let mut steps = 0;
        while !self.commands.is_empty() {
            self.run_next();
            steps += 1;
        }
        steps
    }

    // Run at most `max` steps. Returns the number of steps taken.
    pub fn run_until(&mut self, max: usize) -> usize {
        let mut steps = 0;
        while steps < max && !self.commands.is_empty() {
            self.run_next();
            steps += 1;
        }
        steps
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

        s.push(3);
        s.push(7);
        s.run(Command::Mult); // 3 * 7
        assert_eq!(s.pop(), 21);
        assert_eq!(s.data.len(), 0);

        s.push(7);
        s.push(3);
        s.run(Command::Div); // 7 / 3
        assert_eq!(s.pop(), 2);
        assert_eq!(s.data.len(), 0);

        s.push(7);
        s.run(Command::Dup); // 7 -> 7 7
        assert_eq!(s.pop(), 7);
        assert_eq!(s.pop(), 7);
        assert_eq!(s.data.len(), 0);

        s.push(7);
        s.push(3);
        s.run(Command::Swap); // 7 3 -> 3 7
        assert_eq!(s.pop(), 7);
        assert_eq!(s.pop(), 3);
        assert_eq!(s.data.len(), 0);

        // We can run whole programs (sequences of commands)
        let prog = [Prog::D(10), Prog::D(2), Prog::C(Command::Div), Prog::C(Command::Dup)];
        s.queue_program(&prog);
        assert_eq!(s.run_all(), 4);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.data.len(), 0);
        assert_eq!(s.commands.len(), 0);

        // We can run segments of programs, then resume then later
        s.queue_program(&prog);
        assert_eq!(s.run_until(2), 2);
        assert_eq!(s.data.len(), 2);
        assert_eq!(s.run_all(), 2);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.pop(), 5);
        assert_eq!(s.data.len(), 0);
        assert_eq!(s.commands.len(), 0);
    }
}
