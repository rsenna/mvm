use phf::phf_map;

type Register64 = u64;

pub struct Registers64 {
    pub pc: Register64,
    pub data: [Register64; 32]
}

pub struct RegisterMapItem {
    pos: i32,
    abi: &'static str,
    id: &'static str,
    saved_by: SavedBy
}

pub enum SavedBy {
    None,
    Caller,
    Callee
}

static REGISTERS_BASE_MAP: phf::Map<&'static str, RegisterMapItem> = phf_map! {
     "pc" => RegisterMapItem { pos: -1, abi:      "pc", id:         "", saved_by: SavedBy::None   },
   "zero" => RegisterMapItem { pos:  0, abi:    "zero", id:      "x01", saved_by: SavedBy::None   },
     "ra" => RegisterMapItem { pos:  1, abi:      "ra", id:      "x02", saved_by: SavedBy::Caller },
     "sp" => RegisterMapItem { pos:  2, abi:      "sp", id:      "x03", saved_by: SavedBy::Callee },
     "gp" => RegisterMapItem { pos:  3, abi:      "gp", id:      "x04", saved_by: SavedBy::None   },
     "tp" => RegisterMapItem { pos:  4, abi:      "tp", id:      "x05", saved_by: SavedBy::None   },
     "t0" => RegisterMapItem { pos:  5, abi:      "t0", id:      "x06", saved_by: SavedBy::Caller },
     "t1" => RegisterMapItem { pos:  6, abi:      "t1", id:      "x07", saved_by: SavedBy::Caller },
     "t2" => RegisterMapItem { pos:  7, abi:      "t2", id:      "x08", saved_by: SavedBy::Caller },
"s0 / fp" => RegisterMapItem { pos:  8, abi: "s0 / fp", id:      "x09", saved_by: SavedBy::Callee },
     "s1" => RegisterMapItem { pos:  9, abi:      "s1", id:      "x10", saved_by: SavedBy::Callee },
     "a0" => RegisterMapItem { pos: 10, abi:      "a0", id:      "x11", saved_by: SavedBy::Caller },
     "a1" => RegisterMapItem { pos: 11, abi:      "a1", id:      "x12", saved_by: SavedBy::Caller },
     "a2" => RegisterMapItem { pos: 12, abi:      "a2", id:      "x13", saved_by: SavedBy::Caller },
     "a3" => RegisterMapItem { pos: 13, abi:      "a3", id:      "x14", saved_by: SavedBy::Caller },
     "a4" => RegisterMapItem { pos: 14, abi:      "a4", id:      "x15", saved_by: SavedBy::Caller },
     "a5" => RegisterMapItem { pos: 15, abi:      "a5", id:      "x16", saved_by: SavedBy::Caller },
     "a6" => RegisterMapItem { pos: 16, abi:      "a6", id:      "x17", saved_by: SavedBy::Caller },
     "a7" => RegisterMapItem { pos: 17, abi:      "a7", id:      "x18", saved_by: SavedBy::Caller },
     "s2" => RegisterMapItem { pos: 18, abi:      "s2", id:      "x19", saved_by: SavedBy::Callee },
     "s3" => RegisterMapItem { pos: 19, abi:      "s3", id:      "x20", saved_by: SavedBy::Callee },
     "s4" => RegisterMapItem { pos: 20, abi:      "s4", id:      "x21", saved_by: SavedBy::Callee },
     "s5" => RegisterMapItem { pos: 21, abi:      "s5", id:      "x22", saved_by: SavedBy::Callee },
     "s6" => RegisterMapItem { pos: 22, abi:      "s6", id:      "x23", saved_by: SavedBy::Callee },
     "s7" => RegisterMapItem { pos: 23, abi:      "s7", id:      "x24", saved_by: SavedBy::Callee },
     "s8" => RegisterMapItem { pos: 24, abi:      "s8", id:      "x25", saved_by: SavedBy::Callee },
     "s9" => RegisterMapItem { pos: 25, abi:      "s9", id:      "x26", saved_by: SavedBy::Callee },
    "s10" => RegisterMapItem { pos: 26, abi:     "s10", id:      "x27", saved_by: SavedBy::Callee },
    "s11" => RegisterMapItem { pos: 27, abi:     "s11", id:      "x28", saved_by: SavedBy::Callee },
     "t3" => RegisterMapItem { pos: 28, abi:      "t3", id:      "x29", saved_by: SavedBy::Caller },
     "t4" => RegisterMapItem { pos: 29, abi:      "t4", id:      "x30", saved_by: SavedBy::Caller },
     "t5" => RegisterMapItem { pos: 30, abi:      "t5", id:      "x31", saved_by: SavedBy::Caller },
     "t6" => RegisterMapItem { pos: 31, abi:      "t6", id:      "x32", saved_by: SavedBy::Caller }
};

impl Registers64 {
    pub fn get(&self, reg: &str) -> Register64 {
        if reg == "pc" {
            return self.pc
        }

        let index = REGISTERS_BASE_MAP
            .into_iter()
            .find_map(|(str, it)|
                if str == &reg {
                    Some(it.pos)
                } else {
                    None
                })
            .unwrap();

        self.data[index as usize]
    }
}
