pub enum Species {
    Human,
    Felinoid,
    Lycanthrope,
    Vulpithrope,
}

//Major skills with subskills? Subskills can have multiple parent skills?
//Distilling is both chemistry and cooking
pub enum Skills {
    Metalworking, // machining, smelting, electrical engineering
    Mechanic,     //cars watches
    Gunsmithing,
    Woodworking, //carving, carpentry
    Programming, //hacking, AI,robotics
    Medical,
    Cooking, // brewing, fermenting, distilling

    Tailoring,
    Leatherworking,
    Salvaging,
}

//This should probably be a struct
pub enum Stats {
    Strength,
    Dexterity,
    Endurance,
    Intelligence,
    Perception,
    Charisma,
    Luck,
}
/*
Derived Stats

These are influenced by primary stats and affect gameplay mechanics.

    Health (HP) – Determined by Endurance; affects survivability.
    Stamina (STA) – Affects sprinting, melee swings, and heavy lifting.
    Radiation Resistance (RAD RES) – Determines how well you withstand radiation exposure. Based on Endurance.
    Carry Capacity – Affected by Strength and Endurance.
    Melee Damage – Based on Strength.
    Firearm Accuracy – Based on Perception and Dexterity.
    Stealth – Influenced by Dexterity and Perception.
    Barter Bonus – Influenced by Charisma and Luck.
    Tech Proficiency – Based on Intelligence; affects hacking, robotics, and engineering.
    Dodge Chance – Influenced by Dexterity and Luck.
*/
