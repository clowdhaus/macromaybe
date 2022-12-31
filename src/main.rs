use std::fmt;

use clap::ValueEnum;
use seq_macro::seq;
use serde::{Deserialize, Serialize};

// #[derive(Clone, Copy, Debug, Display, Serialize, Deserialize)]
// pub enum ClusterVersion {
//     #[strum(serialize = "1.20")]
//     V20,
//     #[strum(serialize = "1.21")]
//     V21,
//     #[strum(serialize = "1.22")]
//     V22,
//     #[strum(serialize = "1.23")]
//     V23,
//     #[strum(serialize = "1.24")]
//     V24,
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

//     fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
//         match self {
//             Self::V20 => Some(clap::builder::PossibleValue::new("1.20")),
//             Self::V21 => Some(clap::builder::PossibleValue::new("1.21")),
//             Self::V22 => Some(clap::builder::PossibleValue::new("1.22")),
//             Self::V23 => Some(clap::builder::PossibleValue::new("1.23")),
//             Self::V24 => Some(clap::builder::PossibleValue::new("1.24")),
//         }
//     }
// }

seq!(N in 20..=24 {
    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum ClusterVersion {
        #( V~N, )* // V20
    }

    impl fmt::Display for ClusterVersion {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                #( ClusterVersion::V~N => write!(f, "1.{}", N), )*
                // ClusterVersion::V20 => write!(f, "1.{}", 20),
            }
        }
    }

    impl ValueEnum for ClusterVersion {
        fn value_variants<'a>() -> &'a [Self] {
            &[
                #( Self::V~N, )* // Self::V20,
            ]
        }

        fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
            match self {
                #( Self::V~N => Some(clap::builder::PossibleValue::new(format!("1.{}", N))), )*
                // Self::V20 => Some(clap::builder::PossibleValue::new("1.20")),
            }
        }
    }
});

fn main() {
    println!("{}", ClusterVersion::V24);
}
