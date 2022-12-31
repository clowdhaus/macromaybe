use std::fmt;

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

seq!(N in 20..=24 {
    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    pub enum ClusterVersion {
        #(
            V~N,
        )*
    }

    impl fmt::Display for ClusterVersion {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                #(
                    ClusterVersion::V~N => write!(f, "1.{}", N),
                )*
            }
        }
    }
});

fn main() {
    println!("{}", ClusterVersion::V24);
}
