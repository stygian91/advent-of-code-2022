use std::collections::HashMap;

pub struct Face {
    pub grid: Vec<Vec<char>>,
    pub identity: FaceIdentity,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum FaceIdentity {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

pub struct Cube {
    pub faces: HashMap<FaceIdentity, Face>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AxisDirection {
    XPositive,
    XNegative,
    YPositive,
    YNegative,
}