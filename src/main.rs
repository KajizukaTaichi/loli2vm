fn main() {
    let mut compiler = Compiler {
        stack_index: 0,
        target: "nasm-x86_64".to_string(),
    };
    let bytecodes = Compiler::parse_ir(include_str!("../example.lir")).unwrap();
    let assembly_code = compiler.compile(bytecodes);
    println!("{}", assembly_code.unwrap());
}

enum Instruction {
    Const(i64),
    Add,
    Sub,
    Mul,
}

struct Compiler {
    stack_index: usize,
    target: String,
}

impl Compiler {
    fn parse_ir(source: &str) -> Option<Vec<Instruction>> {
        let mut result = vec![];
        for line in source.lines() {
            if let Some(n) = line.strip_prefix("const ") {
                if let Ok(n) = n.trim().parse() {
                    result.push(Instruction::Const(n));
                } else {
                    return None;
                }
            } else if line == "add" {
                result.push(Instruction::Add);
            } else if line == "sub" {
                result.push(Instruction::Sub);
            } else if line == "mul" {
                result.push(Instruction::Mul);
            } else {
                return None;
            }
        }
        Some(result)
    }

    fn compile(&mut self, bytecodes: Vec<Instruction>) -> Option<String> {
        if self.target == "nasm-x86_64" {
            let mut assembly_code = "section .text\n\tglobal _start\n\n_start:\n".to_string();
            for bytecode in bytecodes {
                match bytecode {
                    Instruction::Const(value) => {
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
                    Instruction::Mul => {
                        assembly_code.push_str(&format!(
                            "\timul r{}, r{}\n",
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
