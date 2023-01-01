use std::fmt;

use clap::{builder::PossibleValue, ValueEnum};
use seq_macro::seq;
use serde::{Deserialize, Serialize};

// #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
// pub enum ClusterVersion {
//     V20,
//     V21,
//     V22,
//     V23,
//     V24,
// }

// impl fmt::Display for ClusterVersion {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             ClusterVersion::V20 => write!(f, "1.20"),
//             ClusterVersion::V21 => write!(f, "1.21"),
//             ClusterVersion::V22 => write!(f, "1.22"),
//             ClusterVersion::V23 => write!(f, "1.23"),
//             ClusterVersion::V24 => write!(f, "1.24"),
//         }
//     }
// }

// impl ValueEnum for ClusterVersion {
//     fn value_variants<'a>() -> &'a [Self] {
//         &[
//             Self::V20,
//             Self::V21,
//             Self::V22,
//             Self::V23,
//             Self::V24,
//         ]
//     }

//     fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
//         match self {
//             Self::V20 => Some(PossibleValue::new("1.20")),
//             Self::V21 => Some(PossibleValue::new("1.21")),
//             Self::V22 => Some(PossibleValue::new("1.22")),
//             Self::V23 => Some(PossibleValue::new("1.23")),
//             Self::V24 => Some(PossibleValue::new("1.24")),
//         }
//     }
// }

seq!(N in 20..=24 {
    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum ClusterVersion {
        #( V~N, )*
    }

    impl fmt::Display for ClusterVersion {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                #( ClusterVersion::V~N => write!(f, "1.{}", N), )*
            }
        }
    }

    impl ValueEnum for ClusterVersion {
        fn value_variants<'a>() -> &'a [Self] {
            &[
                #( Self::V~N, )*
            ]
        }

        fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
            match self {
                #( Self::V~N => Some(PossibleValue::new(format!("1.{}", N))), )*
            }
        }
    }
});

fn main() {
    println!("{}", ClusterVersion::V24);
}
