fn main() {
    let mut compiler = Compiler {
        stack_index: 0,
        target: "nasm-x86_64-macos".to_string(),
    };
    let bytecodes = Compiler::parse_ir(include_str!("../example.sbir")).unwrap();
    let assembly_code = compiler.compile(bytecodes);
    println!("{}", assembly_code.unwrap());
}

const REGISTER_BASE: usize = 8;

enum Instruction {
    Push(i64),
    Add,
    Sub,
    Mul,
    Equal,
    Label(String),
    Jump(String),
    BrIf(String),
}

struct Compiler {
    stack_index: usize,
    target: String,
}

impl Compiler {
    fn parse_ir(source: &str) -> Option<Vec<Instruction>> {
        let mut result = vec![];
        for line in source.lines() {
            if let Some(n) = line.strip_prefix("push") {
                if let Ok(n) = n.trim().parse() {
                    result.push(Instruction::Push(n));
                } else {
                    return None;
                }
            } else if let Some(label) = line.strip_prefix("label") {
                result.push(Instruction::Label(label.trim().to_owned()));
            } else if let Some(label) = line.strip_prefix("jump") {
                result.push(Instruction::Jump(label.trim().to_owned()));
            } else if let Some(label) = line.strip_prefix("jmp_if") {
                result.push(Instruction::BrIf(label.trim().to_owned()));
            } else if line == "add" {
                result.push(Instruction::Add);
            } else if line == "sub" {
                result.push(Instruction::Sub);
            } else if line == "mul" {
                result.push(Instruction::Mul);
            } else if line == "is_eql" {
                result.push(Instruction::Equal);
            } else {
                return None;
            }
        }
        Some(result)
    }

    fn compile(&mut self, bytecodes: Vec<Instruction>) -> Option<String> {
        if self.target == "nasm-x86_64-macos" {
            let mut assembly_code = "section .text\n\tglobal _start\n\n_start:\n".to_string();
            for bytecode in bytecodes {
                match bytecode {
                    Instruction::Push(value) => {
                        assembly_code.push_str(&format!(
                            "\tmov r{}, {}\n",
                            self.stack_index + REGISTER_BASE,
                            value
                        ));
                        self.stack_index += 1;
                    }
                    Instruction::Add => {
                        assembly_code.push_str(&format!(
                            "\tadd r{}, r{}\n",
                            self.stack_index + REGISTER_BASE - 2,
                            self.stack_index + REGISTER_BASE - 1,
                        ));
                        self.stack_index -= 1;
                    }
                    Instruction::Sub => {
                        assembly_code.push_str(&format!(
                            "\tsub r{}, r{}\n",
                            self.stack_index + REGISTER_BASE - 2,
                            self.stack_index + REGISTER_BASE - 1,
                        ));
                        self.stack_index -= 1;
                    }
                    Instruction::Mul => {
                        assembly_code.push_str(&format!(
                            "\timul r{}, r{}\n",
                            self.stack_index + REGISTER_BASE - 2,
                            self.stack_index + REGISTER_BASE - 1,
                        ));
                        self.stack_index -= 1;
                    }
                    Instruction::Equal => {
                        assembly_code.push_str(&format!(
                            "\tcmp r{a}, r{}\n\tsete al\n\tmovzx r{a}, al",
                            self.stack_index + REGISTER_BASE - 1,
                            a = self.stack_index + REGISTER_BASE - 2,
                        ));
                        self.stack_index -= 1;
                    }
                    Instruction::Label(label) => {
                        assembly_code.push_str(&format!("\n{label}:\n",));
                    }
                    Instruction::Jump(label) => {
                        assembly_code.push_str(&format!("\tjmp {label}\n",));
                    }
                    Instruction::BrIf(label) => {
                        assembly_code.push_str(&format!(
                            "\tcmp r{}, 1\n\tje {label}\n",
                            self.stack_index + REGISTER_BASE - 1,
                        ));
                        self.stack_index -= 1;
                    }
                }
            }
            Some(
                assembly_code
                    + &format!(
                        "\n\tmov rax, 0x2000001\n\tmov rdi, r{}\n\tsyscall",
                        self.stack_index + REGISTER_BASE - 1
                    ),
            )
        } else if self.target == "llvm-unknown-unknown" {
            let mut assembly_code = "define i64 @main() {\n\tentry:\n".to_string();
            for bytecode in bytecodes {
                match bytecode {
                    Instruction::Push(value) => {
                        assembly_code.push_str(&format!(
                            "\t%r{} = add i64 0, {}\n",
                            self.stack_index, value
                        ));
                        self.stack_index += 1;
                    }
                    Instruction::Add => {
                        assembly_code.push_str(&format!(
                            "\t%r{} = add i64 %r{}, %r{}\n",
                            self.stack_index,
                            self.stack_index - 2,
                            self.stack_index - 1,
                        ));
                        self.stack_index += 1;
                    }
                    Instruction::Sub => {
                        assembly_code.push_str(&format!(
                            "\t%r{} = sub i64 %r{}, %r{}\n",
                            self.stack_index,
                            self.stack_index - 2,
                            self.stack_index - 1,
                        ));
                        self.stack_index += 1;
                    }
                    Instruction::Mul => {
                        assembly_code.push_str(&format!(
                            "\t%r{} = mul i64 %r{}, %r{}\n",
                            self.stack_index,
                            self.stack_index - 2,
                            self.stack_index - 1,
                        ));
                        self.stack_index += 1;
                    }
                    Instruction::Label(label) => {
                        assembly_code.push_str(&format!("\n{label}:\n"));
                    }
                    Instruction::Jump(label) => {
                        assembly_code.push_str(&format!("\tbr label %{label}\n"));
                    }
                    _ => todo!(),
                }
            }
            Some(assembly_code + &format!("\n\tret i64 %r{}\n}}", self.stack_index - 1))
        } else {
            None
        }
    }
}
