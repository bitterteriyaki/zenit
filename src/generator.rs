use crate::parser::node::NodeExit;

pub struct Generator {
    root: NodeExit,
}

impl Generator {
    pub fn new(root: NodeExit) -> Generator {
        Generator { root }
    }

    pub fn generate(&self) -> String {
        let mut asm = String::new();
        let value = self.root
            .expr
            .integer_literal
            .value
            .as_ref()
            .unwrap();

        asm.push_str("global _start:\n\n");
        asm.push_str("_start:\n");

        asm.push_str("    mov rax, 60\n");
        asm.push_str(format!("    mov rdi, {}\n", value).as_str());
        asm.push_str("    syscall\n");

        asm
    }
}
