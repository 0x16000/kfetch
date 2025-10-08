pub struct Distro {
    pub ascii: Vec<&'static str>,
    pub color: &'static str,
}

pub fn nixos() -> Distro {
    Distro {
	ascii: vec![
            "      \\\\  \\\\ //     ",
            "    ==\\\\__\\\\/ //     ",
            "      //   \\\\//      ",
            "   ==//     //==       ",
            "     //\\\\___//         ",
            "    // /\\\\  \\\\==       ",
            "      // \\\\  \\\\        ",
        ],
        color: "\x1b[36m",
    }
}

pub fn arch() -> Distro {
    Distro {
        ascii: vec![
            "       /\\        ",
            "      /  \\       ",
            "     /\\   \\      ",
            "    /      \\     ",
            "   /   ,,   \\    ",
            "  /   |  |  -\\   ",
            " /_-''    ''-_\\  ",
        ],
        color: "\x1b[34m",
    }
}

pub fn arco() -> Distro {
    Distro {
        ascii: vec![
            "      /\\        ",
            "     /  \\       ",
            "    / /\\ \\      ",
            "   / /  \\ \\     ",
            "  / /    \\ \\    ",
            " / / _____\\ \\   ",
            "/_/  `----.\\_\\  ",
        ],
        color: "\x1b[34m",
    }
}

pub fn artix() -> Distro {
    Distro {
        ascii: vec![
            "      /\\        ",
            "     /  \\       ",
            "    /\\`'.\\     ",
            "   /     ',     ",
            "  /      ,`\\    ",
            " /   ,.'`.  \\   ",
            "/.,'`     `'.\\  ",
        ],
        color: "\x1b[34m",
    }
}

pub fn debian() -> Distro {
    Distro {
        ascii: vec![
            "  _____        ",
            " /  __ \\       ",
            "|  /    |      ",
            "|  \\___-       ",
            "-_             ",
            "  --_           ",
        ],
        color: "\x1b[31m",
    }
}

pub fn endeavour() -> Distro {
    Distro {
        ascii: vec![
            "      /\\        ",
            "    //  \\\\      ",
            "   //    \\ \\    ",
            "  / /     _) )   ",
            " /_/___-- __-    ",
            "  /____--        ",
        ],
        color: "\x1b[36m",
    }
}

pub fn fedora() -> Distro {
    Distro {
                ascii: vec![
            "      ,'''''        ",
            "     |   ,.  |      ",
            "     |  |  '_       ",
            " ,....|  |..        ",
            ".'  ,_;|   ..'      ",
            "|  |   |  |         ",
            "|  ',_,'  |         ",
            " '.     ,'          ",
            "   '''''            ",
        ],
        color: "\x1b[34m",
    }
}

pub fn gentoo() -> Distro {
    Distro {
        ascii: vec![
            "     _-----_       ",
            "    (       \\      ",
            "    \\    0   \\     ",
            "     \\        )    ",
            "     /      _/      ",
            "    (     _-        ",
            "    \\____-          ",
        ],
        color: "\x1b[35m",
    }
}

pub fn mint() -> Distro {
    Distro {
                ascii: vec![
            "     ___________       ",
            "    |_          \\      ",
            "      | | _____ |      ",
            "      | | | | | |      ",
            "      | | | | | |      ",
            "      | \\_____/ |      ",
            "      \\_________/      ",
        ],
        color: "\x1b[32m",
    }
}

pub fn manjaro() -> Distro {
    Distro {
        ascii: vec![
            "  ||||||||| ||||  ",
            "  ||||||||| ||||  ",
            "  ||||      ||||  ",
            "  |||| |||| ||||  ",
            "  |||| |||| ||||  ",
            "  |||| |||| ||||  ",
            "  |||| |||| ||||  ",
        ],
        color: "\x1b[32m",
    }
}

pub fn opensuse() -> Distro {
    Distro {
        ascii: vec![
            "      _______       ",
            "    __|   __ \\      ",
            "         / .\\ \\     ",
            "         \\__/ |     ",
            "       _______|     ",
            "       \\_______     ",
            "    ____________/   ",
        ],
        color: "\x1b[32m",
    }
}

pub fn slackware() -> Distro {
    Distro {
        ascii: vec![
            "      ________       ",
            "     /  ______|      ",
            "     | |______       ",
            "     \\______  \\      ",
            "      ______| |      ",
            "     | |________/    ",
            "     |____________   ",
        ],
        color : "\x1b[34m",
    }
}

pub fn ubuntu() -> Distro {
    Distro {
        ascii: vec![
            "            _            ",
            "        ---(_)           ",
            "     _/  ---  \\          ",
            "    (_) |   |            ",
            "      \\  --- _/          ",
            "         ---(_)           ",
        ],
        color : "\x1b[38;5;208m",
    }
}

pub fn void() -> Distro {
    Distro {
        ascii: vec![
            "                _______        ",
            "             _ \\______ -      ",
            "            | \\  ___  \\ |     ",
            "            | | /   \\ | |     ",
            "            | | \\___/ | |     ",
            "            | \\______ \\_|     ",
            "             -_______\\         ",
        ],
        color: "\x1b[32m",
    }
}

pub fn linux() -> Distro {
    Distro {
        ascii: vec![
            "    .--.",
            "   |o_o |",
            "   |:_/ |",
            "  //   \\ \\",
            " (|     | )",
            "/'\\_   _/`\\",
            "\\___)=(___/",
        ],
        color: "\x1b[33m",
    }
}
