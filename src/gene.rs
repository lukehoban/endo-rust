
pub struct Gene {
    pub name: &'static str,
    pub offset: usize,
    pub length: usize
}

pub fn gene_table() -> Vec<Gene> {
    vec![
        Gene { name: "AAA_genePageTableNr", offset: 0x510, length: 0x18 },
        Gene { name: "M-class-planet", offset: 0x2ccd88, length: 0x3c7f0 },
        Gene { name: "__array_index", offset: 0xc4589, length: 0x18 },
        Gene { name: "__array_value", offset: 0xc45a1, length: 0x18 },
        Gene { name: "__bool", offset: 0xc45e9, length: 0x1 },
        Gene { name: "__bool2", offset: 0xc45ea, length: 0x1 },
        Gene { name: "__funptr", offset: 0xc45b9, length: 0x30 },
        Gene { name: "__int1", offset: 0xc461b, length: 0x1 },
        Gene { name: "__int12", offset: 0xc4628, length: 0xc },
        Gene { name: "__int12_2", offset: 0xc4634, length: 0xc },
        Gene { name: "__int24", offset: 0xc45eb, length: 0x18 },
        Gene { name: "__int24_2", offset: 0xc603, length: 0x18 },
        Gene { name: "__int3", offset: 0xc4625, length: 0x3 },
        Gene { name: "__int48", offset: 0xc4640, length: 0x30 },
        Gene { name: "__int9", offset: 0xc461c, length: 0x9 },
        Gene { name: "acc1", offset: 0xc4541, length: 0x18 },
        Gene { name: "acc2", offset: 0xc4559, length: 0x18 },
        Gene { name: "acc3", offset: 0xc4571, length: 0x18 },
        Gene { name: "activateAdaptationTree", offset: 0x6fce9c, length: 0xb02 },
        Gene { name: "activateGene", offset: 0x6fd99e, length: 0x273 },
        Gene { name: "adapter", offset: 0x252fa1, length: 0x6db },
        Gene { name: "addFunctionsCBF", offset: 0x41b532, length: 0x16ce },
        Gene { name: "addInts", offset: 0x54b1ba, length: 0x325 },
        Gene { name: "DAMAGED", offset: 0x0, length: 0x0 },
        Gene { name: "anticompressant", offset: 0x5580c4, length: 0x2b42 },
        Gene { name: "apple", offset: 0x65f785, length: 0x3fb },
        Gene { name: "appletree", offset: 0xc870e, length: 0x372b },
        Gene { name: "apply1_adaptation", offset: 0x711dc6, length: 0x48 },
        Gene { name: "apply2_adaptation", offset: 0x719633, length: 0x48 },
        Gene { name: "DAMAGED", offset: 0x0, length: 0x0 },
                
    ]
}