pub struct Distro {
    pub ascii: &'static [&'static str],
    pub color: &'static str,
}

mod colors {
    pub const CYAN: &str = "\x1b[36m";
    pub const ARCH_BLUE: &str = "\x1b[38;2;23;147;209m";
    pub const DEBIAN_RED: &str = "\x1b[38;2;206;0;86m";
    pub const ENDEAVOUR_PURPLE: &str = "\x1b[38;2;144;129;187m";
    pub const FEDORA_BLUE: &str = "\x1b[38;2;11;87;164m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const MINT_GREEN: &str = "\x1b[38;2;134;190;67m";
    pub const GREEN: &str = "\x1b[32m";
    pub const OPENSUSE_GREEN: &str = "\x1b[38;2;115;186;37m";
    pub const BLUE: &str = "\x1b[34m";
    pub const UBUNTU_ORANGE: &str = "\x1b[38;2;233;84;32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RASPBIAN_PINK: &str = "\x1b[38;5;125m";
}

macro_rules! distro {
    ($color:expr, $($line:expr),+ $(,)?) => {
        Distro {
            ascii: &[$($line),+],
            color: $color,
        }
    };
}

pub fn cachyos() -> Distro {
    distro!(
        colors::CYAN,
        "   /''''''''''''/   ",
        "  /''''''''''''/    ",
        " /''''''/           ",
        "/''''''/            ",
        "\\......\\            ",
        " \\......\\           ",
        "  \\.............../  ",
        "   \\............./   ",
    )
}

pub fn nixos() -> Distro {
    distro!(
        colors::CYAN,
        "      \\\\   \\\\ //     ",
        "    ==\\\\___\\\\/ //     ",
        "      //    \\\\//      ",
        "   ==//      //==       ",
        "     //\\\\____//         ",
        "    // /\\\\   \\\\==       ",
        "      // \\\\   \\\\        ",
    )
}

pub fn arch() -> Distro {
    distro!(
        colors::ARCH_BLUE,
        "       /\\        ",
        "      /  \\       ",
        "     /\\   \\      ",
        "    /      \\     ",
        "   /   ,,   \\    ",
        "  /   |  |  -\\   ",
        " /_-''    ''-_\\  ",
    )
}

pub fn arco() -> Distro {
    distro!(
        colors::ARCH_BLUE,
        "      /\\        ",
        "     /  \\       ",
        "    / /\\ \\      ",
        "   / /  \\ \\     ",
        "  / /    \\ \\    ",
        " / / _____\\ \\   ",
        "/_/  `----.\\_\\  ",
    )
}

pub fn artix() -> Distro {
    distro!(
        colors::ARCH_BLUE,
        "      /\\        ",
        "     /  \\       ",
        "    /\\`'.\\     ",
        "   /     ',     ",
        "  /      ,`\\    ",
        " /   ,.'`.  \\   ",
        "/.,'`     `'.\\  ",
    )
}

pub fn debian() -> Distro {
    distro!(
        colors::DEBIAN_RED,
        "  _____        ",
        " /  __ \\       ",
        "|  /    |      ",
        "|  \\___-       ",
        "-_             ",
        "  --_           ",
    )
}

pub fn endeavour() -> Distro {
    distro!(
        colors::ENDEAVOUR_PURPLE,
        "      /\\        ",
        "    //  \\\\      ",
        "   //    \\ \\    ",
        "  / /     _) )   ",
        " /_/___-- __-    ",
        "  /____--        ",
    )
}

pub fn fedora() -> Distro {
    distro!(
        colors::FEDORA_BLUE,
        "      ,'''''        ",
        "     |   ,.  |      ",
        "     |  |  '_       ",
        " ,....|  |..        ",
        ".'  ,_;|   ..'      ",
        "|  |   |  |         ",
        "|  ',_,'  |         ",
        " '.     ,'          ",
        "   '''''            ",
    )
}

pub fn gentoo() -> Distro {
    distro!(
        colors::MAGENTA,
        "     _-----_       ",
        "    (       \\      ",
        "    \\    0   \\     ",
        "     \\        )    ",
        "     /      _/      ",
        "    (     _-        ",
        "    \\____-          ",
    )
}

pub fn mint() -> Distro {
    distro!(
        colors::MINT_GREEN,
        "     ___________       ",
        "    |_          \\      ",
        "      | | _____ |      ",
        "      | | | | | |      ",
        "      | | | | | |      ",
        "      | \\_____/ |      ",
        "      \\_________/      ",
    )
}

pub fn manjaro() -> Distro {
    distro!(
        colors::GREEN,
        "  ||||||||| ||||  ",
        "  ||||||||| ||||  ",
        "  ||||      ||||  ",
        "  |||| |||| ||||  ",
        "  |||| |||| ||||  ",
        "  |||| |||| ||||  ",
        "  |||| |||| ||||  ",
    )
}

pub fn opensuse() -> Distro {
    distro!(
        colors::OPENSUSE_GREEN,
        "      _______       ",
        "    __|   __ \\      ",
        "         / .\\ \\     ",
        "         \\__/ |     ",
        "       _______|     ",
        "       \\_______     ",
        "    ____________/   ",
    )
}

pub fn slackware() -> Distro {
    distro!(
        colors::BLUE,
        "      ________       ",
        "     /  ______|      ",
        "     | |______       ",
        "     \\______  \\      ",
        "      ______| |      ",
        "     | |________/    ",
        "     |____________   ",
    )
}

pub fn ubuntu() -> Distro {
    distro!(
        colors::UBUNTU_ORANGE,
        "            _            ",
        "        ---(_)           ",
        "     _/  ---  \\          ",
        "    (_) |   |            ",
        "      \\  --- _/          ",
        "         ---(_)           ",
    )
}

pub fn void() -> Distro {
    distro!(
        colors::GREEN,
        "                _______        ",
        "             _ \\______ -      ",
        "            | \\  ___  \\ |     ",
        "            | | /   \\ | |     ",
        "            | | \\___/ | |     ",
        "            | \\______ \\_|     ",
        "             -_______\\         ",
    )
}

pub fn raspbian() -> Distro {
    distro!(
        colors::RASPBIAN_PINK,
        "              __  __",
        "             (_\\)(/_)",
        "             (_(__)_)",
        "            (_(_)(_)_)",
        "             (_(__)_)",
        "               (__)",
    )
}

pub fn linux() -> Distro {
    distro!(
        colors::YELLOW,
        "    .--.",
        "   |o_o |",
        "   |:_/ |",
        "  //   \\ \\",
        " (|     | )",
        "/'\\_   _/`\\",
        "\\___)=(___/",
    )
}
