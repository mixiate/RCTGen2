#[derive(Clone, Copy, Debug, strum::EnumCount, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPosition {
    TopCorner,
    LeftCorner,
    RightCorner,
    BottomCorner,
    Centre,
    TopLeftSide,
    TopRightSide,
    BottomLeftSide,
    BottomRightSide,
}

impl SupportPosition {
    pub fn rotate(self, rotation: usize) -> Self {
        use SupportPosition as P;
        use strum::EnumCount as _;

        static ROTATION_MAP: [[SupportPosition; 4]; SupportPosition::COUNT] = [
            [P::TopCorner, P::RightCorner, P::BottomCorner, P::LeftCorner], // TopCorner
            [P::LeftCorner, P::TopCorner, P::RightCorner, P::BottomCorner], // LeftCorner
            [P::RightCorner, P::BottomCorner, P::LeftCorner, P::TopCorner], // RightCorner
            [P::BottomCorner, P::LeftCorner, P::TopCorner, P::RightCorner], // BottomCorner
            [P::Centre, P::Centre, P::Centre, P::Centre],                   // Centre
            [P::TopLeftSide, P::TopRightSide, P::BottomRightSide, P::BottomLeftSide], // TopLeftSide
            [P::TopRightSide, P::BottomRightSide, P::BottomLeftSide, P::TopLeftSide], // TopRightSide
            [P::BottomLeftSide, P::TopLeftSide, P::TopRightSide, P::BottomRightSide], // BottomLeftSide
            [P::BottomRightSide, P::BottomLeftSide, P::TopLeftSide, P::TopRightSide], // BottomRightSide
        ];

        ROTATION_MAP[self as usize][rotation % 4]
    }
}

#[derive(Clone, Copy, Debug, strum::EnumCount, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetalSupportType {
    Tubes,
    Fork,
    Boxed,
    Stick,
    Thick,
    Truss,
    TubesInverted,
    BoxedCoated,
}
