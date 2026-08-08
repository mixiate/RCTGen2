use strum::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize, EnumCount, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Colour {
    Black,
    Grey,
    White,
    DarkPurple,
    LightPurple,
    BrightPurple,
    DarkBlue,
    LightBlue,
    IcyBlue,
    Teal,
    Aquamarine,
    SaturatedGreen,
    DarkGreen,
    MossGreen,
    BrightGreen,
    OliveGreen,
    DarkOliveGreen,
    BrightYellow,
    Yellow,
    DarkYellow,
    LightOrange,
    DarkOrange,
    LightBrown,
    SaturatedBrown,
    DarkBrown,
    SalmonPink,
    BordeauxRed,
    SaturatedRed,
    BrightRed,
    DarkPink,
    BrightPink,
    LightPink,
    DarkOliveDark,
    DarkOliveLight,
    SaturatedBrownLight,
    BordeauxRedDark,
    BordeauxRedLight,
    GrassGreenDark,
    GrassGreenLight,
    OliveDark,
    OliveLight,
    SaturatedGreenLight,
    TanDark,
    TanLight,
    DullPurpleLight,
    DullGreenDark,
    DullGreenLight,
    SaturatedPurpleDark,
    SaturatedPurpleLight,
    OrangeLight,
    AquaDark,
    MagentaLight,
    DullBrownDark,
    DullBrownLight,
    Invisible,
    Void,
}

pub static COLOUR_RAMPS: [[u8; 12]; Colour::COUNT] = [
    [10, 10, 10, 10, 10, 11, 12, 13, 14, 15, 16, 17],             // Black
    [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21],             // Grey
    [13, 14, 15, 16, 17, 18, 19, 20, 21, 21, 21, 21],             // White
    [118, 118, 118, 119, 119, 120, 121, 122, 122, 123, 124, 124], // DarkPurple
    [118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129], // LightPurple
    [154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165], // BrightPurple
    [130, 130, 130, 131, 131, 132, 133, 134, 134, 135, 136, 136], // DarkBlue
    [130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141], // LightBlue
    [133, 134, 135, 136, 137, 138, 139, 140, 140, 141, 141, 141], // IcyBlue
    [190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201], // Teal
    [191, 193, 195, 196, 197, 198, 199, 200, 200, 201, 201, 201], // Aquamarine
    [94, 94, 94, 95, 95, 96, 97, 98, 98, 99, 100, 100],           // SaturatedGreen
    [142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153], // DarkGreen
    [70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81],             // MossGreen
    [94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105],       // BrightGreen
    [82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93],             // OliveGreen
    [22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33],             // DarkOliveGreen
    [48, 49, 50, 51, 52, 53, 54, 55, 56, 56, 57, 57],             // BrightYellow
    [46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57],             // Yellow
    [46, 46, 46, 47, 47, 48, 49, 50, 51, 52, 53, 53],             // DarkYellow
    [178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189], // LightOrange
    [178, 178, 178, 179, 179, 180, 181, 182, 182, 183, 184, 184], // DarkOrange
    [34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45],             // LightBrown
    [34, 34, 34, 34, 35, 36, 37, 38, 39, 40, 41, 42],             // SaturatedBrown
    [214, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224], // DarkBrown
    [106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117], // SalmonPink
    [58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69],             // BordeauxRed
    [166, 166, 166, 167, 167, 168, 169, 170, 170, 171, 172, 172], // SaturatedRed
    [166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177], // BrightRed
    [202, 202, 202, 203, 203, 204, 205, 206, 206, 207, 208, 208], // DarkPink
    [202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213], // BrightPink
    [63, 64, 65, 66, 67, 68, 68, 176, 69, 177, 177, 177],         // LightPink
    [22, 22, 22, 23, 23, 24, 25, 26, 26, 27, 28, 28],             // DarkOliveDark
    [24, 25, 26, 27, 28, 29, 30, 31, 32, 32, 33, 33],             // DarkOliveLight
    [36, 37, 38, 39, 40, 41, 42, 43, 44, 44, 45, 45],             // SaturatedBrownLight
    [58, 58, 58, 59, 59, 60, 61, 62, 62, 63, 64, 64],             // BordeauxRedDark
    [60, 61, 62, 63, 64, 65, 66, 67, 68, 68, 69, 69],             // BordeauxRedLight
    [70, 70, 70, 70, 71, 72, 73, 74, 75, 76, 77, 78],             // GrassGreenDark
    [72, 73, 74, 75, 76, 77, 78, 79, 80, 80, 81, 81],             // GrassGreenLight
    [82, 82, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91],             // OliveDark
    [84, 85, 86, 87, 88, 89, 90, 91, 92, 92, 93, 93],             // OliveLight
    [96, 97, 98, 99, 100, 101, 102, 103, 104, 104, 105, 105],     // SaturatedGreenLight
    [106, 106, 106, 106, 107, 108, 109, 110, 111, 112, 113, 114], // TanDark
    [108, 109, 110, 111, 112, 113, 114, 115, 116, 116, 117, 117], // TanLight
    [120, 121, 122, 123, 124, 125, 126, 127, 128, 128, 129, 129], // DullPurpleLight
    [142, 142, 142, 143, 143, 144, 145, 146, 146, 147, 148, 148], // DullGreenDark
    [144, 145, 146, 147, 148, 149, 150, 151, 152, 152, 153, 153], // DullGreenLight
    [154, 154, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163], // SaturatedPurpleDark
    [156, 157, 158, 159, 160, 161, 162, 163, 164, 164, 165, 165], // SaturatedPurpleLight
    [180, 181, 182, 183, 184, 185, 186, 187, 188, 188, 189, 189], // OrangeLight
    [190, 190, 190, 190, 191, 192, 193, 194, 195, 196, 197, 198], // AquaDark
    [204, 205, 206, 207, 208, 209, 210, 211, 212, 212, 213, 213], // MagentaLight
    [214, 214, 214, 215, 215, 216, 217, 218, 218, 219, 220, 220], // DullBrownDark
    [216, 217, 218, 219, 220, 221, 222, 223, 224, 224, 225, 225], // DullBrownLight
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],                         // Invisible
    [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],             // Void
];
