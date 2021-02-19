use rdkit::romol::Mol;

#[test]
fn mol_props() {
    // test no-mol
    let mol = Mol::from_smiles("xxx");
    assert!(mol.is_none());

    // (+/-)-tryptophane
    if let Some(mol) = Mol::from_smiles("NC(Cc1c[nH]c2ccccc12)C(=O)O") {
        assert_eq!(mol.num_atoms(), 15);
        let mw = mol.mol_wt();
        assert!(mw > 204.228 && mw < 204.230);
        assert_eq!(mol.num_hbd(), 3);
        assert_eq!(mol.num_hba(), 2);
        assert_eq!(mol.num_rotatable_bonds(), 3);
        assert_eq!(mol.num_hetero_atoms(), 4);
        assert_eq!(mol.num_rings(), 2);
        assert_eq!(mol.sssr(), 2);
        let fcsp3 = mol.fraction_csp3();
        assert!(fcsp3 > 0.181 && fcsp3 < 0.182);
        let clogp = mol.clogp();
        assert!(clogp > 1.122 && clogp < 1.123);
        assert_eq!(mol.tpsa(), 79.11);
    } else {
        panic!("Mol should be Some.");
    }
}

#[test]
fn mol_default() {
    // test default (no-struct)
    let mol = Mol::from_smiles("xxx").unwrap_or_default();
    assert_eq!(mol.num_atoms(), 1);
    assert_eq!(mol.mol_wt(), 0.0);
}

#[test]
fn mol_to_smiles() {
    let smi = "c1ccccc1C(=O)NC";
    let m = Mol::from_smiles(&smi).unwrap();
    // println!("{}", m.to_smiles());
    assert_eq!(m.to_smiles(), "CNC(=O)c1ccccc1");
}
