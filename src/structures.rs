pub mod structures {
    use std::mem::ManuallyDrop;

    type Flow = Vec<FlowStructure>;

    /* Registers */
    enum Register {
        RAX,
        RBX,
        RCX,
        RDX,
        RSI,
        RDI,
        RSP,
        RBP,
        NIL
    }

    /* Data Type Vars */
    struct GeneralData {
        t: DataType,
        d: AnyData
    }

    enum DataType {
        Uint32,
        Uint64,
        Int32,
        Int64,
        Float,
        Double,
        String,
        Char,
        Register
    }

    struct AnyData {
        pub uint32: u32,
        pub uint64: u64,
        pub int32: i32,
        pub int64: i64,
        pub float: f32,
        pub double: f64,
        pub string: ManuallyDrop<String>,
        pub char: char,
        pub register: Register
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: Register::NIL
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
                register: val
            }
        }
    }

    /* Env Vars */
    struct Flags {
        pub zf: bool
    }



    pub struct EnvVars {
        flags: Flags,
        registers: Vec<GeneralData>,
        pub pc: i64,
        stack: Vec<GeneralData>
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
    }

    /* Flow Structure */
    enum OpCode {
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
        COUNT
    }

    struct FlowStructure {
        op_code: OpCode,
        arguments: Vec<GeneralData>
    }

    /* General Vars */
    pub struct GeneralStructure {
        env: EnvVars,
        flow: Flow
    }
}