use xi_rope::Rope;

pub struct Gene {
    pub name: &'static str,
    pub offset: usize,
    pub length: usize,
    pub code: Option<String>
}

fn new_gene(name: &'static str, offset: usize, length: usize) -> Gene {
    Gene {name: name, offset: offset, length: length, code: None }
}

pub fn gene_table(dna: &Rope) -> Vec<Gene> {
    let mut genes = vec![
        new_gene("AAA_genePageTableNr", 0x510,  0x18 ),
        new_gene("M-class-planet", 0x2ccd88,  0x3c7f0 ),
        new_gene("__array_index", 0xc4589,  0x18 ),
        new_gene("__array_value", 0xc45a1,  0x18 ),
        new_gene("__bool", 0xc45e9,  0x1 ),
        new_gene("__bool2", 0xc45ea,  0x1 ),
        new_gene("__funptr", 0xc45b9,  0x30 ),
        new_gene("__int1", 0xc461b,  0x1 ),
        new_gene("__int12", 0xc4628,  0xc ),
        new_gene("__int12_2", 0xc4634,  0xc ),
        new_gene("__int24", 0xc45eb,  0x18 ),
        new_gene("__int24_2", 0xc603,  0x18 ),
        new_gene("__int3", 0xc4625,  0x3 ),
        new_gene("__int48", 0xc4640,  0x30 ),
        new_gene("__int9", 0xc461c,  0x9 ),
        new_gene("acc1", 0xc4541,  0x18 ),
        new_gene("acc2", 0xc4559,  0x18 ),
        new_gene("acc3", 0xc4571,  0x18 ),
        new_gene("activateAdaptationTree", 0x6fce9c,  0xb02 ),
        new_gene("activateGene", 0x6fd99e,  0x273 ),
        new_gene("adapter", 0x252fa1,  0x6db ),
        new_gene("addFunctionsCBF", 0x41b532,  0x16ce ),
        new_gene("addInts", 0x54b1ba,  0x325 ),
        new_gene("DAMAGED", 0x0,  0x0 ),
        new_gene("anticompressant", 0x5580c4,  0x2b42 ),
        new_gene("apple", 0x65f785,  0x3fb ),
        new_gene("appletree", 0xc870e,  0x372b ),
        new_gene("apply1_adaptation", 0x711dc6,  0x48 ),
        new_gene("apply2_adaptation", 0x719633,  0x48 ),
        new_gene("DAMAGED", 0x0,  0x0 ),
        
        new_gene("balloon", 0x6f31b6,  0x1a83 ),
        new_gene("beginRelativeMode", 0x6f08a9,  0x2af ),
        new_gene("bioAdd_adaptation", 0x710436,  0x258 ),
        new_gene("bioMorphPerturb", 0xc9229,  0x588 ),
        new_gene("bioMul_adaptation", 0x719153,  0x498 ),
        new_gene("bioSucc_adaptation", 0x71068e,  0xf0 ),
        new_gene("bioZero_adapatation", 0x7103a6,  0x90 ),
        new_gene("biomorph_adaptation", 0x717323,  0x1ce0 ),
        new_gene("blueZoneStart", 0x7295a1,  0x0 ),
        new_gene("bmu", 0xdaedb,  0x5806 ),
        new_gene("bresenhamArray", 0xc886b,  0x78 ),
        new_gene("bresenhamIndex", 0xc88e3,  0x18 ),
        new_gene("bresenhamRadius", 0xc8853,  0x18 ),
        new_gene("bridge", 0x6f0e03,  0xb28 ),
        new_gene("bridge-close", 0x56426f,  0x1350 ),
        new_gene("bride-far", 0x44c262,  0x14f0 ),
        new_gene("cachedFastCircle", 0x3fdd8a,  0x362e ),
        new_gene("cachedFastCorner", 0x544d6f,  0xeb0 ),
        new_gene("cachedFastEllipse", 0x45e69e,  0xb1af ),
        new_gene("caravan", 0x5706a4,  0x1365 ),
        new_gene("caravan-axis", 0x5a58e9,  0x67d ),
        new_gene("caravan-door", 0x56f483,  0x62f ),
        new_gene("caravan-frame", 0x6ee1ca,  0x26c7 ),
        new_gene("caravan-wheel", 0x2abe9c,  0xd07 ),
        new_gene("caravan-window1", 0x1ad921,  0xa62 ),
        new_gene("caravan-window2", 0x23d82a,  0xb34 ),
        new_gene("cargobox", 0x21edd5,  0x6022 ),
        new_gene("caseNat_adaptation", 0x71ba1d,  0x48 ),
        new_gene("casePair_adptation", 0x7163ca,  0x48 ),
        new_gene("casePictureDescr_adaptation", 0x72954c,  0x48 ),
        
        new_gene("caseVar1_adaptation", 0x713860,  0x48 ),
        new_gene("caseVar2_adaptation", 0x70d83c,  0x48 ),
        new_gene("cbfArray", 0x0ca12a,  0x5a0 ),
        new_gene("charColorCallback", 0xc86eb,  0x30 ),
        new_gene("charColorCallback", 0xc871b,  0x30 ),
        new_gene("charCounter", 0xc8d00,  0x18 ),
        new_gene("charIndexArray", 0xc8d30,  0x4b0 ),
        new_gene("charIndexOffset", 0xc8d18,  0x18 ),
        new_gene("charInfo_Tempus-Bold-Huge_", 0x79e9f,  0x46 ),
        new_gene("charInfo_Tempus-Bold-Huge_L", 0x79ee5,  0x196 ),
        new_gene("charInfo_Tempus-Bold-Huge_M", 0x7a97b,  0xe51 ),
        new_gene("checkIntegrity", 0x3e9f1a,  0x868 ),
        new_gene("checksum", 0x21bcc7,  0xd15 ),
        new_gene("chick", 0x541d0e,  0x3049 ),
        new_gene("cloak-night", 0x652673,  0x6bf ),
        new_gene("cloak-rain", 0x309590,  0x3484d ),
        new_gene("closureArguments", 0x0c97b2,  0x960 ),
        new_gene("closureIndex", 0x0ca112,  0x18 ),
        new_gene("cloud", 0x60fea4,  0x1962 ),
        new_gene("clouds", 0x5c909f,  0xbc1 ),
        new_gene("DAMAGED", 0x0,  0x0 ),
        new_gene("colorBlack", 0x23adf8,  0x172 ),
        new_gene("colorBlue", 0x25f3c4,  0x172 ),
        new_gene("colorByIndex", 0x1a4e72,  0x64a ),
        new_gene("colorCyan", 0x3c8584,  0x172 ),
        new_gene("colorDuckBrown", 0x0d92ad,  0x596 ),
        new_gene("colorDuckOrange", 0x21edd5,  0x1f4 ),
        new_gene("colorDuckYellow", 0x6d730d,  0x208 ),
        new_gene("colorGermanyYellow", 0x65e3c5,  0x244 ),
        new_gene("colorGreen", 0x35cd8d,  0x172 ),
    ];
    
    for i in 0..genes.len() {
        let mut gene = &mut genes[i];
        let bases = dna.clone().slice(13615 + gene.offset, 13615 + gene.offset + gene.length);
        if bases.len() >= 10 {
            gene.code = Some(String::from(bases.slice(3, 10)))
        }
    }
    
    genes
}