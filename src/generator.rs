use crate::nodes::NodeExit;

pub struct Generator {
    root: NodeExit,
}

impl Generator {
    pub fn new(root: NodeExit) -> Generator {
        Generator { root }
    }

    pub fn generate(&self) -> String {
        let mut output = String::new();
        let value = self.root.expr.integer.value.as_ref().unwrap();

        output.push_str("global _start:\n");
        output.push_str("_start:\n");
        output.push_str("    mov rax, 60\n");
        output.push_str(format!("    mov rdi, {}\n", value).as_str());
        output.push_str("    syscall\n");

        output
    }
}
