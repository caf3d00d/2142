mod registers {
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    #[derive(Clone, Copy, Debug, FromPrimitive)]
    pub enum Register {
        RAX,
        RBX,
        RCX,
        RDX,
        RSI,
        RDI,
        RSP,
        RBP,
        NIL,
    }

    impl Register {
        pub fn is_reg(name: &String) -> bool {
            let archm: Vec<&str> = vec!["R", "E", ""];
            let regs: Vec<&str> = vec!["AX", "BX", "CX", "DX", "SI", "DI", "SP", "BP"];
            let mut reg_table: Vec<String> = Vec::with_capacity(archm.len() * regs.len());
            for reg in regs {
                for &modi in &archm {
                    reg_table.push(format!("{}{}", modi, reg));
                }
            }

            reg_table.contains(&name.to_uppercase())
        }

        pub fn from_string(name: &String) -> Option<Register> {
            let archm: Vec<&str> = vec!["R", "E", ""];
            let regs: Vec<&str> = vec!["AX", "BX", "CX", "DX", "SI", "DI", "SP", "BP"];
            let mut reg_table: Vec<String> = Vec::with_capacity(archm.len() * regs.len());
            for reg in regs {
                for &modi in &archm {
                    reg_table.push(format!("{}{}", modi, reg));
                }
            }
            let p: String = name.to_uppercase();
            let i = reg_table.iter().position(|ins| &p == ins);
            match i {
                Some(idx) => FromPrimitive::from_usize(idx / archm.len()),
                None => None,
            }
        }
    }
}

mod data_types {
    use crate::structures::registers::Register;
    use std::fmt::{Display, Formatter};
    use std::mem::ManuallyDrop;

    pub struct GeneralData {
        pub t: DataType,
        pub d: AnyData,
    }

    impl Display for GeneralData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.t {
                DataType::Uint32 => write!(f, "{}", self.d.uint32),
                DataType::Uint64 => write!(f, "{}", self.d.uint64),
                DataType::Int32 => write!(f, "{}", self.d.int32),
                DataType::Int64 => write!(f, "{}", self.d.int64),
                DataType::Float => write!(f, "{}", self.d.float),
                DataType::Double => write!(f, "{}", self.d.double),
                DataType::String => write!(f, "{}", self.d.string.as_str()),
                DataType::Char => write!(f, "{}", self.d.char),
                DataType::Register => write!(f, "{:?}", self.d.register),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum DataType {
        Uint32,
        Uint64,
        Int32,
        Int64,
        Float,
        Double,
        String,
        Char,
        Register,
    }

    #[derive(Debug, Clone)]
    pub struct AnyData {
        pub uint32: u32,
        pub uint64: u64,
        pub int32: i32,
        pub int64: i64,
        pub float: f32,
        pub double: f64,
        pub string: ManuallyDrop<String>,
        pub char: char,
        pub register: Register,
    }

    impl From<u32> for AnyData {
        fn from(val: u32) -> Self {
            AnyData {
                uint32: val,
                uint64: 0,
                int32: 0,
                int64: 0,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<u64> for AnyData {
        fn from(val: u64) -> Self {
            AnyData {
                uint32: 0,
                uint64: val,
                int32: 0,
                int64: 0,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<i32> for AnyData {
        fn from(val: i32) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: val,
                int64: 0,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<i64> for AnyData {
        fn from(val: i64) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: 0,
                int64: val,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<f32> for AnyData {
        fn from(val: f32) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: 0,
                int64: 0,
                float: val,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<f64> for AnyData {
        fn from(val: f64) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: 0,
                int64: 0,
                float: 0.0,
                double: val,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<&String> for AnyData {
        fn from(val: &String) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: 0,
                int64: 0,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from(val)),
                char: ' ',
                register: Register::NIL,
            }
        }
    }

    impl From<char> for AnyData {
        fn from(val: char) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: 0,
                int64: 0,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: val,
                register: Register::NIL,
            }
        }
    }

    impl From<Register> for AnyData {
        fn from(val: Register) -> Self {
            AnyData {
                uint32: 0,
                uint64: 0,
                int32: 0,
                int64: 0,
                float: 0.0,
                double: 0.0,
                string: ManuallyDrop::new(String::from("")),
                char: ' ',
                register: val,
            }
        }
    }
}

mod flow_structure {
    use crate::structures::data_types::GeneralData;
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    #[derive(Debug, FromPrimitive, Clone, Copy, PartialEq)]
    pub enum OpCode {
        MOV,
        PUSH,
        POP,
        ADD,
        SUB,
        MUL,
        DIV,
        MOD,
        CMP,
        JNE,
        JMP,
        JE,
        INC,
        OR,
        AND,
        XOR,
        CALL,
        RET,
        STDOUT,
        STDIN,
        PNL,
        MALLOC,
        FREE,
        COUNT,
    }

    impl OpCode {
        pub fn isop(name: &String) -> bool {
            let mut istr: Vec<String> = Vec::with_capacity(OpCode::COUNT as usize);

            for i in 0..(OpCode::COUNT as i32) {
                let oc: OpCode = FromPrimitive::from_i32(i).unwrap();
                istr.push(format!("{:?}", oc))
            }
            let p: String = name.to_uppercase();
            istr.contains(&p)
        }

        pub fn from_string(name: &String) -> Option<OpCode> {
            let mut istr: Vec<String> = Vec::with_capacity(OpCode::COUNT as usize);

            for i in 0..(OpCode::COUNT as i32) {
                let oc: OpCode = FromPrimitive::from_i32(i).unwrap();

                istr.push(format!("{:?}", oc))
            }

            let primitive = name.to_uppercase();

            let i = istr.iter().position(|ins| &primitive == ins);
            match i {
                Some(idx) => FromPrimitive::from_usize(idx),
                None => None,
            }
        }
    }

    pub struct FlowStructure {
        pub op_code: OpCode,
        pub arguments: Vec<GeneralData>,
    }

    pub trait IstrTraits {
        fn mov(&mut self, register: &GeneralData, any: &GeneralData);
        fn pop(&mut self, register: &GeneralData);
        fn push(&mut self, any: GeneralData);
        fn add(&mut self, left: &GeneralData, right: &GeneralData);
        fn sub(&mut self, left: &GeneralData, right: &GeneralData);
        fn mul(&mut self, left: &GeneralData, right: &GeneralData);
        fn div(&mut self, left: &GeneralData, right: &GeneralData);
        fn modu(&mut self, left: &GeneralData, right: &GeneralData);
        fn jmp(&mut self, address: &GeneralData);
        fn jne(&mut self, address: &GeneralData);
        fn je(&mut self, address: &GeneralData);
        fn pnl(&mut self, any: &GeneralData);
        fn cmp(&mut self, left: &GeneralData, right: &GeneralData);
    }
}

mod env_vars {
    use crate::structures::data_types::{AnyData, DataType, GeneralData};
    use crate::structures::flow_structure::{FlowStructure, IstrTraits, OpCode};
    use crate::structures::registers::Register;

    struct Flags {
        pub zf: bool,
    }

    pub struct EnvVars {
        flags: Flags,
        registers: Vec<GeneralData>,
        pub pc: i64,
        //stack: Vec<GeneralData>,
    }

    impl EnvVars {
        pub fn init() -> Self {
            let mut this = EnvVars {
                flags: Flags { zf: false },
                registers: Vec::with_capacity(Register::NIL as usize),
                pc: 0,
                //stack: Vec::new(),
            };

            for _ in 0..this.registers.capacity() {
                this.registers.push(GeneralData {
                    t: DataType::Int32,
                    d: AnyData::from(0),
                });
            }

            this
        }

        pub fn execute_istr(&mut self, istr: &FlowStructure) {
            match istr.op_code {
                OpCode::MOV => self.mov(&istr.arguments[0], &istr.arguments[1]),
                OpCode::PUSH => todo!(),
                OpCode::POP => todo!(),
                OpCode::ADD => self.add(&istr.arguments[0], &istr.arguments[1]),
                OpCode::SUB => todo!(),
                OpCode::MUL => todo!(),
                OpCode::DIV => todo!(),
                OpCode::MOD => self.modu(&istr.arguments[0], &istr.arguments[1]),
                OpCode::CMP => self.cmp(&istr.arguments[0], &istr.arguments[1]),
                OpCode::JNE => self.jne(&istr.arguments[0]),
                OpCode::JMP => self.jmp(&istr.arguments[0]),
                OpCode::JE => self.je(&istr.arguments[0]),
                OpCode::INC => todo!(),
                OpCode::OR => todo!(),
                OpCode::AND => todo!(),
                OpCode::XOR => todo!(),
                OpCode::CALL => todo!(),
                OpCode::RET => todo!(),
                OpCode::STDOUT => todo!(),
                OpCode::STDIN => todo!(),
                OpCode::PNL => self.pnl(&istr.arguments[0]),
                OpCode::MALLOC => todo!(),
                OpCode::FREE => todo!(),
                OpCode::COUNT => todo!(),
            }
        }
    }

    impl IstrTraits for EnvVars {
        fn mov(&mut self, register: &GeneralData, any: &GeneralData) {
            assert!(register.t == DataType::Register);
            let r_id = register.d.register.clone() as usize;
            if any.t == DataType::Register {
                let a_id = any.d.register.clone() as usize;
                self.registers[r_id].t = self.registers[a_id].t.clone();
                self.registers[r_id].d = self.registers[a_id].d.clone();
            } else {
                self.registers[r_id].t = any.t.clone();
                self.registers[r_id].d = any.d.clone();
            }
        }

        fn pop(&mut self, register: &GeneralData) {
            todo!()
        }

        fn push(&mut self, any: GeneralData) {
            todo!()
        }

        fn add(&mut self, left: &GeneralData, right: &GeneralData) {
            if left.t == DataType::Register {
                let r_id = left.d.register as usize;
                let data = &self.registers[r_id];

                if right.t == DataType::Register {
                    let o_id = right.d.register as usize;
                    let o_data = &self.registers[o_id];
                    assert!(o_data.t == data.t);

                    match data.t {
                        DataType::Uint32 => self.registers[r_id].d.uint32 += o_data.d.uint32,
                        DataType::Uint64 => self.registers[r_id].d.uint64 += o_data.d.uint64,
                        DataType::Int32 => self.registers[r_id].d.int32 += o_data.d.int32,
                        DataType::Int64 => self.registers[r_id].d.int64 += o_data.d.int64,
                        DataType::Float => self.registers[r_id].d.float += o_data.d.float,
                        DataType::Double => self.registers[r_id].d.double += o_data.d.double,
                        DataType::Char => {
                            self.registers[r_id].d.char =
                                (self.registers[r_id].d.char as u8 + o_data.d.char as u8) as char
                        }
                        //TODO string + register
                        _ => {}
                    }
                } else {
                    assert!(right.t == data.t);

                    match data.t {
                        DataType::Uint32 => {
                            self.registers[r_id].d.uint32 += right.d.uint32;
                        }
                        DataType::Uint64 => {
                            self.registers[r_id].d.uint64 += right.d.uint64;
                        }
                        DataType::Int32 => {
                            self.registers[r_id].d.int32 += right.d.int32;
                        }
                        DataType::Int64 => {
                            self.registers[r_id].d.int64 += right.d.int64;
                        }
                        DataType::Float => {
                            self.registers[r_id].d.float += right.d.float;
                        }
                        DataType::Double => {
                            self.registers[r_id].d.double += right.d.double;
                        }
                        // DataType::String => {
                        //     self.registers[r_id].d.string += right.d.string.clone();
                        // }
                        DataType::Char => {
                            self.registers[r_id].d.char =
                                (self.registers[r_id].d.char as u8 + right.d.char as u8) as char;
                        }
                        // DataType::Register => {
                        //     self.registers[r_id].d.register += right.d.register;
                        // }
                        _ => {}
                    }
                }
            }
        }

        fn sub(&mut self, left: &GeneralData, right: &GeneralData) {
            todo!()
        }

        fn mul(&mut self, left: &GeneralData, right: &GeneralData) {
            todo!()
        }

        fn div(&mut self, left: &GeneralData, right: &GeneralData) {
            todo!()
        }

        fn modu(&mut self, left: &GeneralData, right: &GeneralData) {
            let mut left_data: &GeneralData = left;
            let mut right_data: &GeneralData = right;

            let is_reg: bool = left.t == DataType::Register;
            let r_id: usize = left.d.register.clone() as usize;

            if is_reg {
                left_data = &self.registers[r_id];
            }
            if right.t == DataType::Register {
                right_data = &self.registers[right.d.register.clone() as usize];
            }
            assert!(left_data.t == right_data.t);

            match left_data.t {
                DataType::Uint32 => {
                    let res: u32 = left_data.d.uint32 % right_data.d.uint32;

                    if is_reg {
                        self.registers[r_id].d.uint32 = res;
                        self.flags.zf = res == 0u32;
                    }
                }
                DataType::Uint64 => {
                    let res: u64 = left_data.d.uint64 % right_data.d.uint64;

                    if is_reg {
                        self.registers[r_id].d.uint64 = res;
                        self.flags.zf = res == 0u64;
                    }
                }
                DataType::Int32 => {
                    let res: i32 = left_data.d.int32 % right_data.d.int32;

                    if is_reg {
                        self.registers[r_id].d.int32 = res;
                        self.flags.zf = res == 0i32;
                    }
                }
                DataType::Int64 => {
                    let res: i64 = left_data.d.int64 % right_data.d.int64;

                    if is_reg {
                        self.registers[r_id].d.int64 = res;
                        self.flags.zf = res == 0i64;
                    }
                }
                DataType::Float => {
                    let res: f32 = left_data.d.float % right_data.d.float;

                    if is_reg {
                        self.registers[r_id].d.float = res;
                        self.flags.zf = res == 0f32;
                    }
                }
                DataType::Double => {
                    let res: f64 = left_data.d.double % right_data.d.double;

                    if is_reg {
                        self.registers[r_id].d.double = res;
                        self.flags.zf = res == 0f64;
                    }
                }
                _ => assert!(false, "Wrong type"),
            }
        }

        fn jmp(&mut self, address: &GeneralData) {
            if address.t == DataType::Register {
                let o_id: usize = address.d.register.clone() as usize;
                let o_data: &GeneralData = &self.registers[o_id];
                assert!(o_data.t == DataType::Int64);
                self.pc = address.d.int64 - 1;
            } else {
                assert!(address.t == DataType::Int64);
                self.pc = address.d.int64 - 1;
            }
        }

        fn jne(&mut self, address: &GeneralData) {
            if address.t == DataType::Register {
                let r_id: usize = address.d.register.clone() as usize;
                let r_data: &GeneralData = &self.registers[r_id];
                assert!(r_data.t == DataType::Int64);
                if !self.flags.zf {
                    self.pc = r_data.d.int64 - 1;
                }
            } else {
                assert!(address.t == DataType::Int64);
                if !self.flags.zf {
                    self.pc = address.d.int64 - 1;
                }
            }
        }

        fn je(&mut self, address: &GeneralData) {
            if address.t == DataType::Register {
                let r_data: &GeneralData = &self.registers[address.d.register as usize];
                assert!(r_data.t == DataType::Int64);
                if self.flags.zf {
                    self.pc = r_data.d.int64 - 1
                }
            } else {
                assert!(address.t == DataType::Int64);
                if self.flags.zf {
                    self.pc = address.d.int64 - 1
                }
            }
        }

        fn pnl(&mut self, any: &GeneralData) {
            if any.t == DataType::Register {
                let r_id = any.d.register as usize;
                println!("{}", self.registers[r_id]);
            } else {
                println!("{}", any);
            }
        }

        fn cmp(&mut self, left: &GeneralData, right: &GeneralData) {
            let mut l_data: &GeneralData = left;
            let mut r_data: &GeneralData = right;

            if left.t == DataType::Register {
                l_data = &self.registers[left.d.register as usize];
            }
            if right.t == DataType::Register {
                r_data = &self.registers[right.d.register as usize];
            }
            assert!(l_data.t == r_data.t);
            match l_data.t {
                DataType::Uint32 => {
                    self.flags.zf = l_data.d.uint32 == r_data.d.uint32;
                }
                DataType::Uint64 => {
                    self.flags.zf = l_data.d.uint64 == r_data.d.uint64;
                }
                DataType::Int32 => {
                    self.flags.zf = l_data.d.int32 == r_data.d.int32;
                }
                DataType::Int64 => {
                    self.flags.zf = l_data.d.int64 == r_data.d.int64;
                }
                DataType::Float => {
                    self.flags.zf = l_data.d.float == r_data.d.float;
                }
                DataType::Double => {
                    self.flags.zf = l_data.d.double == r_data.d.double;
                }
                DataType::String => {
                    self.flags.zf = l_data.d.string == r_data.d.string;
                }
                DataType::Char => {
                    self.flags.zf = l_data.d.char == r_data.d.char;
                }
                DataType::Register => {
                    self.flags.zf = false;
                }
            }
        }
    }
}

pub mod parser {
    use crate::structures::data_types::DataType;
    use std::io::Read;

    #[derive(Debug)]
    pub enum ParserError {
        EOF,
        NAC,
    }

    pub struct Parser {
        stream: Vec<char>,
        pub tokens: Vec<String>,
    }

    impl Parser {
        pub fn init(string: String) -> Self {
            let vec: Vec<char> = string.chars().rev().take(string.len()).collect();
            Parser {
                stream: vec,
                tokens: Vec::new(),
            }
        }

        fn read(&self) -> Result<char, ParserError> {
            if self.stream.len() <= 0 {
                return Err(ParserError::EOF);
            }
            let c: char = self.stream[self.stream.len() - 1];

            Ok(c)
        }

        fn consume_char(&mut self) -> Result<char, ParserError> {
            if self.stream.len() <= 0 {
                return Err(ParserError::EOF);
            }

            let chr = self.stream.pop().unwrap();

            Ok(chr)
        }

        fn read_string(&mut self) -> String {
            let mut str: String = String::new();
            let mut stack: Vec<char> = vec![self.consume_char().unwrap()];

            while stack.len() > 0 {
                let c: char = self.consume_char().unwrap();

                if c == '"' {
                    stack.pop();
                    break;
                }
                str.push(c);
            }

            str
        }

        fn read_comment(&mut self) -> String {
            let mut comment: String = String::new();
            let trim: [char; 2] = ['\n', '\r'];
            let mut c: char = self.consume_char().unwrap();

            while self.stream.len() > 0 && !trim.contains(&c) {
                comment.push(c);
                c = self.consume_char().unwrap();
            }
            comment
        }

        fn read_char(&mut self) -> String {
            let mut str = String::new();
            let mut stack: Vec<char> = vec![self.consume_char().unwrap()];

            while stack.len() > 0 {
                let c = self.consume_char().unwrap();

                if c == '\'' {
                    stack.pop();
                    continue;
                }
                str.push(c);
            }

            str
        }

        fn read_raw(&mut self) -> String {
            let mut raw: String = String::new();
            let trim: [char; 5] = [' ', ',', '\n', '\r', '\t'];
            let mut c: char = self.consume_char().unwrap();

            while self.stream.len() > 0 && !trim.contains(&c) {
                raw.push(c);
                c = self.consume_char().unwrap();
            }

            raw
        }

        fn next(&mut self) -> Option<String> {
            let c = self.read().unwrap();
            match c {
                '"' => Some(self.read_string()),
                ';' => Some(self.read_comment()),
                '\'' => Some(self.read_char()),
                _ => {
                    let raw = self.read_raw();

                    if raw.is_empty() {
                        return None;
                    }

                    Some(raw)
                }
            }
        }

        pub fn parse(&mut self) -> &Vec<String> {
            while self.stream.len() > 0 {
                match self.next() {
                    Some(token) => self.tokens.push(token),
                    None => continue,
                }
            }

            &self.tokens
        }
    }
}

pub mod tokens {
    use crate::structures::data_types::{AnyData, DataType};
    use crate::structures::flow_structure::OpCode;
    use crate::structures::registers::Register;
    use std::fmt::{Display, Formatter};

    pub type TokensData = AnyData;

    pub enum Tokens {
        CHECKPOINT(String),
        GOTO(String),
        DATA(DataType, TokensData),
        INSTRUCTION(OpCode),
        REGISTER(Register),
        COMMENT(String),
    }

    impl Display for Tokens {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Tokens::CHECKPOINT(str) => f.write_fmt(format_args!("<Checkpoint {}>", str)),
                Tokens::GOTO(str) => f.write_fmt(format_args!("<Goto {}>", str)),
                Tokens::DATA(t, d) => match t {
                    DataType::Uint32 => f.write_fmt(format_args!("<Uint32 {}>", d.uint32)),
                    DataType::Uint64 => f.write_fmt(format_args!("<Uint64 {}>", d.uint64)),
                    DataType::Int32 => f.write_fmt(format_args!("<Int32 {}>", d.int32)),
                    DataType::Int64 => f.write_fmt(format_args!("<Int64 {}>", d.int64)),
                    DataType::Float => f.write_fmt(format_args!("<Float {}>", d.float)),
                    DataType::Double => f.write_fmt(format_args!("<Double {}>", d.double)),
                    DataType::String => f.write_fmt(format_args!("<String {}>", d.string.as_str())),
                    DataType::Char => f.write_fmt(format_args!("<Char {}>", d.char)),
                    DataType::Register => f.write_fmt(format_args!("<Register {:?}>", d.register)),
                },
                Tokens::INSTRUCTION(istr) => f.write_fmt(format_args!("<Instruction {:?}>", istr)),
                Tokens::REGISTER(reg) => f.write_fmt(format_args!("<Register {:?}>", reg)),
                Tokens::COMMENT(str) => f.write_fmt(format_args!("<Comment \"{}\"", str)),
            }
        }
    }
}

mod stoi {
    pub struct Stoi {}

    impl Stoi {
        fn stoi(str: &String, r: i64) -> i64 {
            let c_table: Vec<char> = vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
            ];
            let mut chars = str.chars();
            let mut mlt: i64 = -1;
            let mut val: i64 = 0;

            if !(chars.next() == Some('-')) {
                mlt *= -1;
                chars = str.chars();
            }
            let mut d: i64 = 1;
            loop {
                match chars.next_back() {
                    Some(v) => match c_table.binary_search(&v) {
                        Ok(i) => {
                            val += i as i64 * d;
                            d *= r;
                        }
                        Err(_) => break,
                    },
                    None => break,
                }
            }
            val * mlt
        }

        pub fn to_int(str: &String) -> Option<i64> {
            let prefix: String = str.chars().take(2).collect();
            let base: usize = match prefix.as_str() {
                "0b" => 2,
                "0o" => 8,
                "0x" => 16,
                _ => 10,
            };
            let chars: Vec<char> = vec![
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
            ];
            let sub = &chars[0..base];
            let mut cs = str.chars();
            let fc = cs.next();
            if !(fc == Some('-')) {
                cs = str.chars();
            }
            if base != 10 {
                cs.next();
                cs.next();
            }
            for c in cs {
                if !sub.contains(&c) {
                    return None;
                }
            }
            Some(Stoi::stoi(str, base as i64))
        }
    }
}

pub mod tokenizer {
    use crate::structures::data_types::{AnyData, DataType};
    use crate::structures::registers::Register;
    // use crate::structures::data_types::DataType::Register;
    use crate::structures::flow_structure::OpCode;
    use crate::structures::parser::Parser;
    use crate::structures::stoi::Stoi;
    use crate::structures::tokens::Tokens;

    pub struct Tokenizer {
        parser: Parser,
        pos: usize,
        tokens: Vec<Tokens>,
        cp: Vec<String>,
        lop: Option<OpCode>,
    }

    impl Tokenizer {
        pub fn init(parsed_arg: Parser) -> Self {
            Tokenizer {
                parser: parsed_arg,
                pos: 0,
                tokens: Vec::new(),
                cp: Vec::new(),
                lop: None,
            }
        }

        fn iscp(token: &String) -> bool {
            token.starts_with(":")
        }

        fn iscomment(token: &String) -> bool {
            token.starts_with(";")
        }

        fn isgoto(&self, tok: &String) -> bool {
            let cp_name: String = format!(":{}", tok);
            let jop = vec![OpCode::JMP, OpCode::JNE, OpCode::JE];

            match self.lop {
                Some(op) => self.cp.contains(&cp_name) && jop.contains(&op),
                None => false,
            }
        }

        fn next(&mut self) -> Tokens {
            let tok: &String = &self.parser.tokens[self.pos];
            self.pos += 1;

            if Register::is_reg(tok) {
                Tokens::REGISTER(Register::from_string(tok).unwrap())
            } else if OpCode::isop(tok) {
                self.lop = OpCode::from_string(tok);
                Tokens::INSTRUCTION(OpCode::from_string(tok).unwrap())
            } else if Tokenizer::iscp(tok) {
                Tokens::CHECKPOINT(String::from(tok))
            } else if Tokenizer::iscomment(tok) {
                Tokens::COMMENT(String::from(tok))
            } else if self.isgoto(tok) {
                Tokens::GOTO(String::from(tok))
            } else {
                match Stoi::to_int(tok) {
                    Some(value) => return Tokens::DATA(DataType::Int64, AnyData::from(value)),
                    None => {}
                }

                Tokens::DATA(DataType::String, AnyData::from(tok))
            }
        }

        fn getcp(&mut self) {
            for t in &self.parser.tokens {
                if Tokenizer::iscp(t) {
                    self.cp.push(String::from(t));
                }
            }
        }

        pub fn tokenize(&mut self) -> &Vec<Tokens> {
            self.getcp();

            while self.pos < self.parser.tokens.len() {
                let tok = self.next();
                self.tokens.push(tok);
            }
            &self.tokens
        }
    }
}

pub mod interpreter {
    use std::collections::{HashMap, VecDeque};
    //use std::intrinsics::pref_align_of;
    use crate::structures::data_types::{AnyData, DataType, GeneralData};
    use crate::structures::flow_structure::{FlowStructure, OpCode};
    use crate::structures::tokens::Tokens;

    pub struct Interpreter {}

    type InterpretedCode = Vec<FlowStructure>;

    impl Interpreter {
        fn cp_pos(tokens: &Vec<Tokens>) -> HashMap<String, usize> {
            let mut i: usize = 0;
            let mut cp: HashMap<String, usize> = HashMap::new();

            for t in tokens {
                match t {
                    Tokens::CHECKPOINT(cpo) => {
                        let mut chars = cpo.chars();
                        chars.next();
                        cp.insert(String::from(chars.as_str()), i);
                    }
                    Tokens::INSTRUCTION(_) => {
                        i += 1;
                        continue;
                    }
                    _ => continue,
                }
            }
            cp
        }

        pub fn interpret(tokens: &Vec<Tokens>) -> InterpretedCode {
            let cp = Interpreter::cp_pos(tokens);
            let mut code: InterpretedCode = InterpretedCode::new();
            let mut args: VecDeque<GeneralData> = VecDeque::new();
            let mut queued_istr: OpCode = OpCode::COUNT;

            for token in tokens {
                match token {
                    Tokens::CHECKPOINT(_) => continue,
                    Tokens::COMMENT(_) => continue,
                    Tokens::GOTO(c_pos) => match cp.get(c_pos) {
                        Some(i) => {
                            args.push_back(GeneralData {
                                t: DataType::Int64,
                                d: AnyData::from(*i as i64),
                            });
                        }
                        None => {}
                    },
                    Tokens::DATA(t, d) => match t {
                        DataType::Uint32 => args.push_back(GeneralData {
                            t: DataType::Uint32,
                            d: AnyData::from(d.uint32),
                        }),
                        DataType::Uint64 => args.push_back(GeneralData {
                            t: DataType::Uint64,
                            d: AnyData::from(d.uint64),
                        }),
                        DataType::Int32 => args.push_back(GeneralData {
                            t: DataType::Int32,
                            d: AnyData::from(d.int32),
                        }),
                        DataType::Int64 => args.push_back(GeneralData {
                            t: DataType::Int64,
                            d: AnyData::from(d.int64),
                        }),
                        DataType::Float => args.push_back(GeneralData {
                            t: DataType::Float,
                            d: AnyData::from(d.float),
                        }),
                        DataType::Double => args.push_back(GeneralData {
                            t: DataType::Double,
                            d: AnyData::from(d.double),
                        }),
                        DataType::String => args.push_back(GeneralData {
                            t: DataType::String,
                            d: AnyData::from(&d.string.to_string()),
                        }),
                        DataType::Char => args.push_back(GeneralData {
                            t: DataType::Char,
                            d: AnyData::from(d.char),
                        }),
                        DataType::Register => args.push_back(GeneralData {
                            t: DataType::Register,
                            d: AnyData::from(d.register),
                        }),
                    },
                    Tokens::REGISTER(reg) => args.push_back(GeneralData {
                        t: DataType::Register,
                        d: AnyData::from(*reg),
                    }),
                    Tokens::INSTRUCTION(istr) => {
                        if queued_istr != OpCode::COUNT {
                            let mut vec: Vec<GeneralData> = Vec::with_capacity(args.len());
                            while args.len() > 0 {
                                vec.push(args.pop_front().unwrap());
                            }
                            code.push(FlowStructure {
                                op_code: queued_istr,
                                arguments: vec,
                            });
                        }
                        queued_istr = *istr
                    }
                }
            }
            if args.len() > 0 {
                let mut vec: Vec<GeneralData> = Vec::with_capacity(args.len());
                while args.len() > 0 {
                    vec.push(args.pop_front().unwrap());
                }
                code.push(FlowStructure {
                    op_code: queued_istr,
                    arguments: vec,
                });
            }

            code
        }
    }
}

pub mod structures {
    // use crate::structures::data_types::GeneralData;
    use crate::structures::env_vars::EnvVars;
    use crate::structures::flow_structure::FlowStructure;

    type Flow = Vec<FlowStructure>;

    /* General Vars */
    pub struct GeneralStructure {
        env: EnvVars,
        flow: Flow,
    }

    impl GeneralStructure {
        pub fn init(flow: Flow) -> Self {
            GeneralStructure {
                env: EnvVars::init(),
                flow: flow,
            }
        }

        pub fn run(&mut self) {
            while (self.env.pc as usize) < self.flow.len() {
                let istr = &self.flow[self.env.pc as usize];
                self.env.execute_istr(istr);
                self.env.pc += 1;
            }
        }

        pub fn next(&mut self, flow: Flow) {
            self.env.pc = 0;
            self.flow = flow;

            self.run();
        }
    }
}
