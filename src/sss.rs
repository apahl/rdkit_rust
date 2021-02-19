use super::romol::Mol;
use cpp::{cpp, cpp_class};

// cpp_class!(pub unsafe struct _PairVector as "RDKit::MatchVectType");
cpp_class!(pub unsafe struct _RdSSSVector as "std::vector<RDKit::MatchVectType>");

pub struct RdSSSVector {
    v: _RdSSSVector,
    // idx: usize,  // needed for the Iterator
}

impl RdSSSVector {
    pub fn len(&self) -> usize {
        let v = &self.v;
        cpp!(unsafe [v as "std::vector<RDKit::MatchVectType>*"] -> usize as "size_t" {
            return v->size();
        })
    }
}

// impl Iterator for RdSSSVector {
//     type Item = _PairVector;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx < self.len() {
//             let v = &self.v;
//             let idx = self.idx;
//             let result = cpp!(unsafe [v as "std::vector<int>*", idx as "size_t"] -> i32 as "int" {
//                 return v->at(idx);
//             });
//             self.idx += 1;
//             Some(result)
//         } else {
//             None
//         }
//     }
// }

impl Mol {
    pub fn substruct_match(&self, q: &Mol) -> RdSSSVector {
        let ptr_m = self.ptr;
        let ptr_q = q.ptr;
        let v = cpp!(unsafe [ptr_m as "const RDKit::ROMol*", ptr_q as "const RDKit::ROMol*"] -> _RdSSSVector as  "std::vector<RDKit::MatchVectType>" {
            std::vector<RDKit::MatchVectType> matches = RDKit::SubstructMatch(*ptr_m, *ptr_q);
            return matches;
        });
        // RdSSSVector { v, idx: 0 }
        RdSSSVector { v }
    }

    pub fn has_substruct_match(&self, q: &Mol) -> bool {
        self.substruct_match(&q).len() > 0
    }
}
