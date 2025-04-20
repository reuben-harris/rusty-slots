// use rand::Rng;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Symbol {
//     Cherry,
//     Lemon,
//     Orange,
//     Plum,
//     Bell,
//     Bar,
// }

// impl Symbol {
//     fn random() -> Self {
//         match rand::thread_rng().gen_range(0..6) {
//             0 => Symbol::Cherry,
//             1 => Symbol::Lemon,
//             2 => Symbol::Orange,
//             3 => Symbol::Plum,
//             4 => Symbol::Bell,
//             _ => Symbol::Bar,
//         }
//     }

//     fn emoji(self) -> &'static str {
//         match self {
//             Symbol::Cherry => "🍒",
//             Symbol::Lemon  => "🍋",
//             Symbol::Orange => "🍊",
//             Symbol::Plum   => "🍇",
//             Symbol::Bell   => "🔔",
//             Symbol::Bar    => "🍫",
//         }
//     }
// }

// fn is_jackpot(reels: [Symbol; 3]) -> bool {
//     reels[0] == reels[1] && reels[1] == reels[2]
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn emoji_mapping_is_correct() {
//         assert_eq!(Symbol::Cherry.emoji(), "🍒");
//         assert_eq!(Symbol::Lemon.emoji(), "🍋");
//         assert_eq!(Symbol::Orange.emoji(), "🍊");
//         assert_eq!(Symbol::Plum.emoji(), "🍇");
//         assert_eq!(Symbol::Bell.emoji(), "🔔");
//         assert_eq!(Symbol::Bar.emoji(), "🍫");
//     }

//     #[test]
//     fn jackpot_detection_works() {
//         assert!(is_jackpot([Symbol::Bar, Symbol::Bar, Symbol::Bar]));
//         assert!(!is_jackpot([Symbol::Bar, Symbol::Bell, Symbol::Bar]));
//         assert!(!is_jackpot([Symbol::Cherry, Symbol::Cherry, Symbol::Lemon]));
//     }

//     #[test]
//     fn random_produces_all_variants() {
//         let mut seen = std::collections::HashSet::new();
//         for _ in 0..200 {
//             seen.insert(Symbol::random());
//             if seen.len() == 6 {
//                 break;
//             }
//         }
//         assert_eq!(seen.len(), 6, "random() did not produce all variants");
//     }
// }
