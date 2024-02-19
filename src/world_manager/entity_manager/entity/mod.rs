use std::ops::{Add, Div, Mul, Neg, Rem, Sub, AddAssign, DivAssign, MulAssign, RemAssign, SubAssign, Range, RangeFrom, RangeFull, RangeTo, Bound, RangeBounds, RangeInclusive, RangeToInclusive};
//module rules;

pub mod player;
pub enum EntityType {
    Error,
    Player(player::Player),
    Mob(),
}

// impl for all types
macro_rules! impl_stat_simple {
    ($structname: ident, Into) => {
        impl Into<$structname> for i32 {
            fn into(self) -> $structname {
                $structname(self)
            }
        }
    };
    ($structname: ident, Display) => {
        impl std::fmt::Display for $structname {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    ($structname: ident, Neg) => {
        impl Neg for $structname {
            type Output = $structname;
        
            fn neg(self) -> Self::Output {
                Self { 0: -self.0 }
            }
        }
    };
    ($structname: ident, Add) => {
        impl Add<$structname> for $structname {
            type Output = $structname;
        
            fn add(self, rhs: $structname) -> Self::Output {
                (self.0 + rhs.0).into()
            }
        }

        impl Add<i32> for $structname {
            type Output = $structname;
        
            fn add(self, rhs: i32) -> Self::Output {
                (self.0 + rhs).into()
            }
        }
    };
    ($structname: ident, Sub) => {
        impl Sub<$structname> for $structname {
            type Output = $structname;
        
            fn sub(self, rhs: $structname) -> Self::Output {
                (self.0 - rhs.0).into()
            }
        }

        impl Sub<i32> for $structname {
            type Output = $structname;
        
            fn sub(self, rhs: i32) -> Self::Output {
                (self.0 - rhs).into()
            }
        }
    };
    ($structname: ident, AddAssign) => {
        impl AddAssign<$structname> for $structname {
            fn add_assign(&mut self, rhs: $structname) {
                self.0 = self.0 + rhs.0
            }
        }

        impl AddAssign<i32> for $structname {
            fn add_assign(&mut self, rhs: i32) {
                self.0 = self.0 + rhs
            }
        }
    };
    ($structname: ident, SubAssign) => {
        impl SubAssign<$structname> for $structname {
            fn sub_assign(&mut self, rhs: $structname) {
                self.0 = self.0 - rhs.0
            }
        }

        impl SubAssign<i32> for $structname {
            fn sub_assign(&mut self, rhs: i32) {
                self.0 = self.0 - rhs
            }
        }
    };
}

// impl all macro
macro_rules! impl_all {
    ($structname: ident) => {
        impl_stat_simple!($structname, Into);
        impl_stat_simple!($structname, Display);
        impl_stat_simple!($structname, Neg);
        impl_stat_simple!($structname, Add);
        impl_stat_simple!($structname, Sub);
        impl_stat_simple!($structname, AddAssign);
        impl_stat_simple!($structname, SubAssign);
    };
}

// impl all
impl_all!(Health);
impl_all!(Mana);
impl_all!(Hunger);
impl_all!(Armor);

pub struct Health(i32);
pub struct Mana(i32);
pub struct Hunger(i32);
pub struct Armor(i32);

pub enum Afflictions {
    None,
    Fire(u8),
    Water(u8),
}

pub struct Entity<'a> {
    pub entity_type: EntityType,
    pub health: Option<Health>,
    pub mana: Option<Mana>,
    pub hunger: Option<Hunger>,
    pub armor: Option<Armor>,
    pub additional_tags: Option<&'a[Afflictions]>,
}
fn increase_health(entity: &mut Entity) {
    if let Some(&Afflictions::Water(water_amount)) = entity.additional_tags.unwrap().iter().find(|tag| matches!(tag, Afflictions::Water(_))) {
        entity.health.as_mut().unwrap().0 += water_amount as i32;
    }
}

