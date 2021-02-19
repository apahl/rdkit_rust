// https://github.com/mystor/rust-cpp
// https://stackoverflow.com/questions/10865957/printf-with-stdstring
// #include <iostream>
// https://github.com/exrook/leds/blob/master/vamp/src/cxx_util/mod.rs
// https://github.com/boncheolgu/tflite-rs/blob/master/src/model/stl/vector.rs

use cpp::cpp;

cpp! {{
    #include <string>
    #include <vector>
    #include "GraphMol/GraphMol.h"
    #include "RDGeneral/RDProps.h"
    #include "GraphMol/SmilesParse/SmilesParse.h"
    #include "GraphMol/SmilesParse/SmilesWrite.h"
    #include "GraphMol/Descriptors/MolDescriptors.h"
    #include "GraphMol/Descriptors/MolSurf.h"
    #include "GraphMol/MolOps.h"
    #include "GraphMol/Substruct/SubstructMatch.h"
}}

pub mod romol;
pub mod sss;

const MAX_SMILES_LEN: usize = 100;
