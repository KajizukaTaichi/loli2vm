fn main() {
    let mut compiler = Compiler {
        stack_index: 0,
        target: "nasm:x86_64".to_string(),
    };
    let bytecodes = Compiler::parse_ir("1 2 add 2 3 4 sub add 1 add add").unwrap();
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
        for line in source.split_whitespace() {
            if let Ok(n) = line.parse::<i64>() {
                result.push(Instruction::Push(n));
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
                        assembly_code.push_str(&format!(
                            "\tmov r{}, {}\n",
                            self.stack_index + 8,
                            value
                        ));
                        self.stack_index += 1;
                    }
                    Instruction::Add => {
                        assembly_code.push_str(&format!(
                            "\tadd r{}, r{}\n",
                            self.stack_index + 8 - 2,
                            self.stack_index + 8 - 1,
                        ));
                        self.stack_index -= 1;
                    }
                    Instruction::Sub => {
                        assembly_code.push_str(&format!(
                            "\tsub r{}, r{}\n",
                            self.stack_index + 8 - 2,
                            self.stack_index + 8 - 1,
                        ));
                        self.stack_index -= 1;
                    }
                }
            }
            Some(
                assembly_code
                    + &format!(
                        "\n\tmov rax, 0x2000001\n\tmov rdi, r{}\n\tsyscall",
                        self.stack_index + 8 - 1
                    ),
            )
        } else {
            None
        }
    }
}
