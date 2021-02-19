use rdkit::romol::Mol;
// use rdkit::sss;

#[test]
fn sss() {
    let m = Mol::from_smiles("c1ccccc1C(=O)NC2CC2").unwrap(); // cyclopropyl benzamide
    let q1 = Mol::from_smiles("C1CC1").unwrap(); // cyclopropane - smiles
    let q2 = Mol::from_smarts("C1CC1").unwrap(); // cyclopropane - smarts
    let q3 = Mol::from_smiles("C1CCC1").unwrap(); // cyclobutane
    assert!(m.has_substruct_match(&q1));
    assert!(m.has_substruct_match(&q2));
    assert!(!m.has_substruct_match(&q3));
}
