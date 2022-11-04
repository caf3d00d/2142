mod registers {
    #[derive(Clone, Copy, Debug)]
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
}

mod data_types {
    use crate::structures::registers::Register;
    use std::mem::ManuallyDrop;

    pub struct GeneralData {
        pub(crate) t: DataType,
        pub(crate) d: AnyData,
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
        DMP,
        MALLOC,
        FREE,
        COUNT,
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
        fn dmp(&mut self, any: &GeneralData);
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
        stack: Vec<GeneralData>,
    }

    impl EnvVars {
        pub fn init() -> Self {
            let mut this = EnvVars {
                flags: Flags { zf: false },
                registers: Vec::with_capacity(Register::NIL as usize),
                pc: 0,
                stack: Vec::new(),
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
                OpCode::MOD => todo!(),
                OpCode::CMP => todo!(),
                OpCode::JNE => todo!(),
                OpCode::JMP => todo!(),
                OpCode::JE => todo!(),
                OpCode::INC => todo!(),
                OpCode::OR => todo!(),
                OpCode::AND => todo!(),
                OpCode::XOR => todo!(),
                OpCode::CALL => todo!(),
                OpCode::RET => todo!(),
                OpCode::STDOUT => todo!(),
                OpCode::STDIN => todo!(),
                OpCode::DMP => todo!(),
                OpCode::MALLOC => todo!(),
                OpCode::FREE => todo!(),
                OpCode::COUNT => todo!(),
            }
        }
    }

    impl IstrTraits for EnvVars {
        fn mov(&mut self, register: &GeneralData, any: &GeneralData) {
            assert_ne!(register.t, DataType::Register);
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
                    let o_id = right.d.register.clone() as usize;
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
                    assert!(right.t == left.t);

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

        fn dmp(&mut self, any: &GeneralData) {
            todo!()
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
    }
}
