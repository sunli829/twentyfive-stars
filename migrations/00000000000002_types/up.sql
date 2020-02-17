CREATE TYPE CARD_RARITY AS ENUM ('COMMON', 'UNCOMMON', 'RARE', 'SUPER_RARE', 'THEME', 'PROMO');
CREATE TYPE CARD_CATEGORY AS ENUM ('CHARACTER', 'BATTLE', 'STRATAGEM');
CREATE TYPE MODE_TYPE AS ENUM ('ALT', 'ALT_1', 'ALT_2', 'BOT', 'COMBINER', 'BODY', 'HEAD', 'UPGRADE_WEAPON', 'UPGRADE_ARMOR', 'UPGRADE_UTILITY');
CREATE TYPE BATTLE_TYPE AS ENUM ('ACTION', 'SECRET_ACTION', 'UPGRADE_WEAPON', 'UPGRADE_ARMOR', 'UPGRADE_UTILITY');
CREATE TYPE BATTLE_ICON AS ENUM ('ORANGE', 'BLUE', 'WHITE', 'GREEN', 'BLACK');
CREATE TYPE CHARACTER_TRAIT AS ENUM ('MELEE', 'RANGED', 'SPECIALIST');
CREATE TYPE FACTION AS ENUM ('AUTOBOT', 'DECEPTICON', 'MERCENARY');
