use std::collections::HashMap;

use super::cube::{FaceIdentity, AxisDirection};
use super::cube::FaceIdentity::*;
use super::cube::AxisDirection::*;

type TransitionInstruction = (FaceIdentity, u8);

pub struct TransitionTable {
    table: HashMap<FaceIdentity, HashMap<AxisDirection, TransitionInstruction>>,
}

lazy_static! {
    pub static ref TABLE: TransitionTable = {
        let mut table = HashMap::new();

        let mut row = HashMap::new();
        row.insert(YNegative, (Back, 0));
        row.insert(YPositive, (Front, 0));
        row.insert(XPositive, (Right, 3));
        row.insert(XNegative, (Left, 1));
        table.insert(Top, row);

        let mut row = HashMap::new();
        row.insert(YNegative, (Top, 0));
        row.insert(YPositive, (Bottom, 0));
        row.insert(XPositive, (Right, 0));
        row.insert(XNegative, (Left, 0));
        table.insert(Front, row);

        let mut row = HashMap::new();
        row.insert(YNegative, (Front, 0));
        row.insert(YPositive, (Back, 0));
        row.insert(XPositive, (Right, 1));
        row.insert(XNegative, (Left, 3));
        table.insert(Bottom, row);

        let mut row = HashMap::new();
        row.insert(YNegative, (Bottom, 0));
        row.insert(YPositive, (Top, 0));
        row.insert(XPositive, (Right, 2));
        row.insert(XNegative, (Left, 2));
        table.insert(Back, row);

        let mut row = HashMap::new();
        row.insert(YNegative, (Top, 3));
        row.insert(YPositive, (Bottom, 1));
        row.insert(XPositive, (Front, 0));
        row.insert(XNegative, (Back, 2));
        table.insert(Left, row);

        let mut row = HashMap::new();
        row.insert(YNegative, (Top, 1));
        row.insert(YPositive, (Bottom, 3));
        row.insert(XPositive, (Back, 2));
        row.insert(XNegative, (Front, 0));
        table.insert(Right, row);

        TransitionTable { table }
    };
}

