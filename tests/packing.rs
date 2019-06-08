//
// packing.rs
// Copyright (C) 2019 Malcolm Ramsay <malramsay64@gmail.com>
// Distributed under terms of the MIT license.
//

use std::fs;

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use serde::Deserialize;
use serde_json;

use packing;
#[allow(unused_imports)]
use packing::traits::*;
use packing::wallpaper::Wallpaper;
use packing::wallpaper::WyckoffSite;
use packing::{
    monte_carlo_best_packing, Cell2, CrystalFamily, FromSymmetry, LineShape, MCVars, OccupiedSite,
    PackedState, Transform2,
};

#[test]
fn test_packing_improves() -> Result<(), &'static str> {
    let square = LineShape::from_radial("Square", vec![1., 1., 1., 1.]).unwrap();

    let wallpaper = Wallpaper {
        name: String::from("p2"),
        family: CrystalFamily::Monoclinic,
    };

    let isopointal = &[WyckoffSite {
        letter: 'd',
        symmetries: vec![
            Transform2::from_operations("x,y").unwrap(),
            Transform2::from_operations("-x,-y").unwrap(),
        ],
        num_rotations: 1,
        mirror_primary: false,
        mirror_secondary: false,
    }];

    let state =
        PackedState::<LineShape, Cell2, OccupiedSite>::initialise(square, wallpaper, isopointal);

    let init_packing = state.score();

    let vars = MCVars {
        kt_start: 0.1,
        kt_finish: 0.001,
        max_step_size: 0.1,
        num_start_configs: 1,
        steps: 100,
        seed: Some(0),
    };

    let final_state = monte_carlo_best_packing(&vars, state);

    let final_packing = final_state?.score();

    assert!(init_packing < final_packing);

    Ok(())
}

#[test]
fn packing_invalid_1() {
    let serialised = fs::read_to_string("tests/packing_invalid_1.json").unwrap();
    let state: PackedState<LineShape, Cell2, OccupiedSite> =
        serde_json::from_str(&serialised).unwrap();

    assert!(state.score().is_err());
}

#[test]
fn packing_invalid_2() {
    let serialised = fs::read_to_string("tests/packing_invalid_2.json").unwrap();
    let state: PackedState<LineShape, Cell2, OccupiedSite> =
        serde_json::from_str(&serialised).unwrap();

    assert!(state.score().is_err());
}
