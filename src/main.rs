fn main() {
    let mut compiler = Compiler {
        stack_index: 8,
        target: "nasm:x86_64".to_string(),
    };
    let bytecodes = vec![
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Push(1),
        Instruction::Sub,
    ];
    let assembly_code = compiler.compile(bytecodes);
    println!("{}", assembly_code.unwrap());
}

enum Instruction {
    Push(i64),
    Add,
    Sub,
}

struct Compiler {
    stack_index: usize,
    target: String,
}

impl Compiler {
    fn parse_ir(source: &str) -> Option<Vec<Instruction>> {
        let mut result = vec![];
        for line in source.lines() {
            if line.starts_with("push") {
                let value = line.split_whitespace().nth(1).unwrap().parse().unwrap();
                result.push(Instruction::Push(value));
            } else if line.starts_with("add") {
                result.push(Instruction::Add);
            } else if line.starts_with("sub") {
                result.push(Instruction::Sub);
            } else {
                return None;
            }
        }
        Some(result)
    }

    fn compile(&mut self, bytecodes: Vec<Instruction>) -> Option<String> {
        let mut assembly_code = "section .text\n\tglobal _start\n\n_start:\n".to_string();
        if self.target == "nasm:x86_64" {
            for bytecode in bytecodes {
                match bytecode {
                    Instruction::Push(value) => {
                        assembly_code
                            .push_str(&format!("\tmov r{}, {}\n", self.stack_index, value));
                        self.stack_index += 1;
                    }
                    Instruction::Add => {
                        assembly_code.push_str(&format!(
                            "\tadd r{}, r{}\n",
                            self.stack_index - 2,
                            self.stack_index - 1,
                        ));
                        self.stack_index -= 1;
                    }
                    Instruction::Sub => {
                        assembly_code.push_str(&format!(
                            "\tsub r{}, r{}\n",
                            self.stack_index - 2,
                            self.stack_index - 1,
                        ));
                        self.stack_index -= 1;
                    }
                }
            }
            Some(
                assembly_code
                    + &format!(
                        "\n\tmov rax, 0x2000001\n\tmov rdi, r{}\n\tsyscall",
                        self.stack_index - 1
                    ),
            )
        } else {
            None
        }
    }
}
