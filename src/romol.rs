use cpp::cpp;
use std::ffi::{CStr, CString};

use super::MAX_SMILES_LEN;

/// The basic Molecule struct
// since RDKit::SmilesToMol returns a pointer to a mol instance (not the instance itself),
// the cpp_class macro mechanism does not work here.
// The following workaround works well but requires manual implementation
// of the Drop trait to deallocate the molecule.
pub struct Mol {
    pub ptr: *mut usize,
}

/// The Default for a Mol is the no-structure ("*")
impl Default for Mol {
    fn default() -> Self {
        Self::from_smiles("*").unwrap()
        // There is probably an easier way to generate a default Mol,
        // but the following does not work:
        // let ptr = cpp!(unsafe [] -> *mut usize as "const RDKit::ROMol*" {
        //     const RDKit::ROMol* mol = new RDKit::ROMol();
        //     // std::cout << mol << std::endl;
        //     return mol;
        // });
        // assert!(!ptr.is_null());
        // Mol { ptr }
    }
}

impl Drop for Mol {
    fn drop(&mut self) {
        // println!("Dropping molecule.");
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] {
            delete ptr;
        });
        // println!("Molecule dropped.");
    }
}

impl Mol {
    /// Create a Mol from a Smiles string.
    pub fn from_smiles(smiles: &str) -> Option<Self> {
        let cstr = CString::new(smiles).unwrap();
        let cstr_ptr = cstr.as_ptr();
        let ptr = cpp!(unsafe [cstr_ptr as "const char *"] -> *mut usize as "const RDKit::ROMol*" {
            std::string smiles = std::string(cstr_ptr);
            RDKit::SmilesParserParams params = RDKit::SmilesParserParams();
            const RDKit::ROMol* mol = RDKit::SmilesToMol(smiles, params);
            // std::cout << mol << std::endl;
            return mol;
        });
        if ptr.is_null() {
            return None;
        }
        Some(Mol { ptr })
    }

    /// Create a Mol from a Smarts string.
    pub fn from_smarts(smarts: &str) -> Option<Self> {
        let cstr = CString::new(smarts).unwrap();
        let cstr_ptr = cstr.as_ptr();
        let ptr = cpp!(unsafe [cstr_ptr as "const char *"] -> *mut usize as "const RDKit::ROMol*" {
            std::string smarts = std::string(cstr_ptr);
            // RDKit::SmilesParserParams params = RDKit::SmilesParserParams();
            const RDKit::ROMol* mol = RDKit::SmartsToMol(smarts);
            // std::cout << mol << std::endl;
            return mol;
        });
        if ptr.is_null() {
            return None;
        }
        Some(Mol { ptr })
    }

    /// Create a Smiles string from a molecule.
    pub fn to_smiles(&self) -> String {
        let mol_ptr = self.ptr;
        let mut buf_ptr = &mut [0u8; MAX_SMILES_LEN + 1];
        unsafe {
            let length = cpp!( [mol_ptr as "const RDKit::ROMol*", MAX_SMILES_LEN as "size_t", mut buf_ptr as "char*"] -> usize as "size_t" {
                std::string smiles = RDKit::MolToSmiles(*mol_ptr);
                std::size_t length = smiles.copy(buf_ptr, std::min(smiles.size(), MAX_SMILES_LEN));
                buf_ptr[length]='\0';
                // std::cout << "SMILES: " << buf_ptr << std::endl;
                return length;
            });
            CStr::from_bytes_with_nul(&buf_ptr[..length + 1])
                .unwrap()
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Calculates the number of heavy atoms.
    pub fn num_atoms(&self) -> u32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> u32 as "unsigned int" {
            return ptr->getNumAtoms();
        })
    }

    /// Calculates the average molecular weight.
    pub fn mol_wt(&self) -> f64 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> f64 as "double" {
            return RDKit::Descriptors::calcAMW(*ptr);
        })
    }

    /// Calculates the number of hydrogen bond donors.
    pub fn num_hbd(&self) -> u32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> u32 as "unsigned int" {
            return RDKit::Descriptors::calcNumHBD(*ptr);
        })
    }

    /// Calculates the number hydrogen bond acceptors.
    pub fn num_hba(&self) -> u32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> u32 as "unsigned int" {
            return RDKit::Descriptors::calcNumHBA(*ptr);
        })
    }

    /// Calculates the number of rotatable bonds.
    pub fn num_rotatable_bonds(&self) -> u32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> u32 as "unsigned int" {
            return RDKit::Descriptors::calcNumRotatableBonds(*ptr);
        })
    }

    /// Calculates the number of hetero atoms.
    pub fn num_hetero_atoms(&self) -> u32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> u32 as "unsigned int" {
            return RDKit::Descriptors::calcNumHeteroatoms(*ptr);
        })
    }

    /// Calculates the number of rings.
    pub fn num_rings(&self) -> u32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> u32 as "unsigned int" {
            return RDKit::Descriptors::calcNumRings(*ptr);
        })
    }

    /// Calculates the smallest set of smallest rings.
    pub fn sssr(&self) -> i32 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> i32 as "int" {
            return RDKit::MolOps::findSSSR(*ptr);
        })
    }

    /// Calculates the fraction of sp3 carbons.
    pub fn fraction_csp3(&self) -> f64 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> f64 as "double" {
            return RDKit::Descriptors::calcFractionCSP3(*ptr);
        })
    }

    /// Calculates LogP.
    pub fn clogp(&self) -> f64 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> f64 as "double" {
            return RDKit::Descriptors::calcClogP(*ptr);
        })
    }

    /// Calculates topological polar surface area.
    pub fn tpsa(&self) -> f64 {
        let ptr = self.ptr;
        cpp!(unsafe [ptr as "const RDKit::ROMol*"] -> f64 as "double" {
            return RDKit::Descriptors::calcTPSA(*ptr);
        })
    }
}
