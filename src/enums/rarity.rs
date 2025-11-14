use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
}

