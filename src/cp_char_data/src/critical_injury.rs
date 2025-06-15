#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalInjury {
    pub name: &'static str,
    pub description: &'static str,
    pub death_save_penalty: i32,
    pub quick_fix: &'static [FixOption],
    pub treatment: &'static [FixOption],
    pub penalties: &'static [Penalty],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixOption {
    pub skill: &'static str,
    pub dv: i32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Penalty {
    pub selector: &'static str,
    pub value: i32
}

pub const BODY_CRIT_INJURIES: &[CriticalInjury] = &[
    CriticalInjury {
        name: "Dismembered Arm",
        description: "The Dismembered Arm is gone. You drop any items in that dismembered arm's hand immediately.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 17
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Dismembered Hand",
        description: "The Dismembered Hand is gone. You drop any items in the dismembered hand immediately.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 17
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Collapsed Lung",
        description: "-2 to MOVE (minimum 1)</br>",
        death_save_penalty: 1,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            }
        ],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 15
            }
        ],
        penalties: &[
            Penalty {
                selector: "move",
                value: -2
            }
        ]
    },
    CriticalInjury {
        name: "Broken Ribs",
        description: "At the end of every Turn wher you move further than 4m/yds on foot, you re-suffer this Critical Injury's Bonus Damage directly to your Hit Points.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            },
            FixOption {
                skill: "Surgery",
                dv: 13
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Broken Arm",
        description: "The Broken Arm cannot be used. You drop any items in that arm's hand immediately.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            },
            FixOption {
                skill: "Surgery",
                dv: 13
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Foreign Object",
        description: "At the end of every Turn where you move further than 4m/yds on foot you re-suffer this Critical Injury's Bonus Damage directly to your Hit Points.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "First Aid",
                dv: 13
            },
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[],
        penalties: &[]
    },
    CriticalInjury {
        name: "Broken Leg",
        description: "-4 to MOVE (minimum 1)",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            },
            FixOption {
                skill: "Surgery",
                dv: 13
            }
        ],
        penalties: &[
            Penalty {
                selector: "move",
                value: -4
            }
        ]
    },
    CriticalInjury {
        name: "Torn Muscle",
        description: "-2 to Melee Attacks",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "First Aid",
                dv: 13
            },
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[],
        penalties: &[
            Penalty {
                selector: "melee",
                value: -2
            }
        ]
    },
    CriticalInjury {
        name: "Spinal Injury",
        description: "Next Turn, you cannot take and Action, but you can still take a Move Action.",
        death_save_penalty: 1,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 15,
            }
        ],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 15
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Crushed Fingers",
        description: "-4 to all actions involving that hand.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13,
            }
        ],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 15
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Dismembered Leg",
        description: "The Dismembered Leg is gone. -6 to MOVE (minimum 1)</br> You cannot dodge attacks.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 17
            }
        ],
        penalties: &[
            Penalty {
                selector: "move",
                value: -6
            }
        ]
    },
];

pub const HEAD_CRIT_INJURIES: &[CriticalInjury] = &[
    CriticalInjury {
        name: "Lost Eye",
        description: "The Lost Eye is gone. -4 to Ranged Attacks & Perception Checks involving vision.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 17
            }
        ],
        penalties: &[
            Penalty {
                selector: "ranged",
                value: -4
            },
            Penalty {
                selector: "perception",
                value: -4
            }
        ]
    },
    CriticalInjury {
        name: "Brain Injury",
        description: "-2 to all Actions.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 17
            }
        ],
        penalties: &[
            Penalty {
                selector: "all",
                value: -2
            }
        ]
    },
    CriticalInjury {
        name: "Damaged Eye",
        description: "-2 to Ranged Attacks & Perception Checks involving vision.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            }
        ],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 13
            }
        ],
        penalties: &[
            Penalty {
                selector: "ranged",
                value: -4
            },
            Penalty {
                selector: "perception",
                value: -4
            }
        ]
    },
    CriticalInjury {
        name: "Concussion",
        description: "-2 to All Actions",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "First Aid",
                dv: 13
            },
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[],
        penalties: &[
            Penalty {
                selector: "all",
                value: -2
            }
        ]
    },
    CriticalInjury {
        name: "Broken Jaw",
        description: "-2 to All Actions involving speech",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 13
            },
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Foreign Object",
        description: "At the end of every Turn where you move further than 4m/yds on foot you re-suffer this Critical Injury's Bonus Damage directly to your Hit Points.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "First Aid",
                dv: 13
            },
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[],
        penalties: &[]
    },
    CriticalInjury {
        name: "Whiplash",
        description: "",
        death_save_penalty: 1,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            },
            FixOption {
                skill: "Surgery",
                dv: 13
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Cracked Skull",
        description: "Aimed Shots to your head multiply the damage that gets through your SP by 3 instead of 2.",
        death_save_penalty: 1,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            }
        ],
        treatment: &[
            FixOption {
                skill: "Paramedic",
                dv: 15
            },
            FixOption {
                skill: "Surgery",
                dv: 15
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Damaged Ear",
        description: "Whenver you move further than 4m/yds on foot in a Turn, you cannot take a Move Action on your next Turn. Additionally you take a -2 to Perception Checks involving hearing.",
        death_save_penalty: 0,
        quick_fix: &[
            FixOption {
                skill: "Paramedic",
                dv: 13
            }
        ],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 13
            }
        ],
        penalties: &[
            Penalty {
                selector: "perception",
                value: -2
            }
        ]
    },
    CriticalInjury {
        name: "Crushed Windpipe",
        description: "You cannot speak.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 15
            }
        ],
        penalties: &[]
    },
    CriticalInjury {
        name: "Lost Ear",
        description: "The Lost Ear is gone. Whenever you move further than 4m/yds on foot in a Turn, you cannot take a Move Action on your next Turn. Additionally you take a -4 to Perception Checks involving hearing.",
        death_save_penalty: 1,
        quick_fix: &[],
        treatment: &[
            FixOption {
                skill: "Surgery",
                dv: 17
            }
        ],
        penalties: &[
            Penalty {
                selector: "perception",
                value: -4
            }
        ]
    }
];