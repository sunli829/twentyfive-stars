table! {
    use diesel::sql_types::*;
    use crate::database::types::{BattleType, BattleIcon};

    battle_cards (id) {
        id -> Uuid,
        card_id -> Uuid,
        title -> Varchar,
        #[sql_name = "type"]
        type_ -> BattleType,
        stars -> Nullable<Int2>,
        icons -> Array<BattleIcon>,
        attack_modifier -> Nullable<Int2>,
        defense_modifier -> Nullable<Int2>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::types::{CardRarity, CardCategory};

    cards (id) {
        id -> Uuid,
        tcg_id -> Varchar,
        rarity -> CardRarity,
        number -> Varchar,
        category -> CardCategory,
        wave_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::types::{CharacterTrait, ModeType};

    character_modes (id) {
        id -> Uuid,
        card_id -> Uuid,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        traits -> Array<CharacterTrait>,
        #[sql_name = "type"]
        type_ -> ModeType,
        stars -> Int4,
        health -> Nullable<Int4>,
        attack -> Nullable<Int4>,
        defense -> Nullable<Int4>,
        attack_modifier -> Nullable<Int4>,
        defense_modifier -> Nullable<Int4>,
    }
}

table! {
    waves (id) {
        id -> Uuid,
        tcg_id -> Varchar,
        name -> Varchar,
        released -> Date,
    }
}

joinable!(battle_cards -> cards (card_id));
joinable!(cards -> waves (wave_id));
joinable!(character_modes -> cards (card_id));

allow_tables_to_appear_in_same_query!(battle_cards, cards, character_modes, waves,);