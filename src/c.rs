//sentences are a combination of enums and entity id's

pub enum Verbs {
    Walk,
    Attack,
}
/*

(ent1 walked to area1)
(ent2 saw ent1 walk to area1)
(ent3 saw ent2 see ent1 walk to area 1)

*/
pub enum SourceOfKnowledge {
    Saw,     // where?
    WasTold, //by who?
    Read,    // from where?
}

// time should be divided into seconds
// commands are composed of partial commands
// each tick progress a partial command
// example: crafting, begin crafting take a second, then to finish crafting takes 20 seconds
//
