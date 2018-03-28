//
// The stack-based programming language
//

// A builtin command to run on the stack
pub enum Command {
    Add,
    Sub,
    Mult,
    Div,
    Dup,
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

        // When popping an empty stack, we get a default value of 0 (not an error)
        assert_eq!(s.pop(), 0);
    }

    #[test]
    fn use_commands() {
        // We can run builtin commands on the stack
        // We can run whole programs (sequences of commands)
        // We can run segments of programs, then resume then later
    }
}
