mod player;

pub enum Afflicton {
    None,
    Water(u8),
    Blood(u8),
    Slime(u8),
    Toxic(u8),
}

pub struct Entity {
    pub health: Option<i32>,

    // needs to be skiped by serde
    pub jet_fuel: Option<u32>,

    // needs to be skiped by serde
    pub velocity: (i32, i32),

    
    pub position: (i32, i32),

    // there has to be a better way
    // needs to be skiped by serde
    pub afflictions: Option<Vec<Afflicton>>
    

}