// This file is part of `i18n_utility-rizzen-yazston` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `i18n_utility-rizzen-yazston` crate.

use i18n_utility::{Category, ScriptData, ScriptDirection};
use std::collections::HashMap;

pub struct DataProvider {
    data: HashMap<String, ScriptData>,
}

impl DataProvider {
    pub fn new() -> DataProvider {
        let mut data = HashMap::<String, ScriptData>::new();
        data.insert(
            "Adlm".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Aghb".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Ahom".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Arab".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Armi".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Armn".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Avst".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Bali".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Bamu".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Bass".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Batk".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Beng".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Bhks".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Bopo".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft,
                ],
                historic: false,
            },
        );
        data.insert(
            "Brah".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Brai".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Bugi".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Buhd".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Cakm".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Cans".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Cari".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft,
                ],
                historic: true,
            },
        );
        data.insert(
            "Cham".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Cher".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Chrs".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Copt".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Cpmn".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Cprt".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Cyrl".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Cyrs".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Deva".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Diak".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Dogr".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Dsrt".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Dupl".to_string(),
            ScriptData {
                category: Category::Special,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Egyp".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft, // Boustrophedon
                ],
                historic: true,
            },
        );
        data.insert(
            "Elba".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Elym".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Ethi".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Gara".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Geok".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Geor".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Glag".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Gong".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Gonm".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Goth".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Gran".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Grek".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft, // Boustrophedon
                ],
                historic: false,
            },
        );
        data.insert(
            "Gujr".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Gukn".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Guru".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Hanb".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft,
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::LeftToRightTopToBottom,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hang".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::LeftToRightTopToBottom, // Historical
                ],
                historic: false,
            },
        );
        data.insert(
            "Hani".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hano".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![
                    ScriptDirection::BottomToTopLeftToRight,
                    ScriptDirection::BottomToTopRightToLeft,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hans".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hant".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hatr".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Hebr".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Hira".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hluw".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft, // Boustrophedon
                    ScriptDirection::LeftToRightTopToBottom, // Monuments
                ],
                historic: true,
            },
        );
        data.insert(
            "Hmng".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Hmnp".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Hikt".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Hung".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Ital".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::TopToBottomRightToLeft,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: true,
            },
        );
        data.insert(
            "Jamo".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Java".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Jpan".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Kali".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Kana".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Kawi".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Khar".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Khmr".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Khoj".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Kits".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![ScriptDirection::RightToLeftTopToBottom],
                historic: true,
            },
        );
        data.insert(
            "Knda".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Kore".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::LeftToRightTopToBottom, // Historical
                ],
                historic: false,
            },
        );
        data.insert(
            "Krai".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Kthi".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Lana".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Laoo".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Latf".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Latg".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Latn".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Lepc".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Limb".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Lina".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Linb".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Lisu".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Lyci".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Lydi".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Mahj".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Maka".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Mand".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Mani".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Marc".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Medf".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Mend".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Merc".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Mero".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Mlym".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Modi".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Mong".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::LeftToRightTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Mroo".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Mtei".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Mult".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Mymr".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Nagm".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Nand".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Narb".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Nbat".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Newa".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Nkoo".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Nshu".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::RightToLeftTopToBottom],
                historic: true,
            },
        );
        data.insert(
            "Ogam".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::BottomToTopLeftToRight,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: true,
            },
        );
        data.insert(
            "Olck".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Onao".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Orkh".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Orya".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Osge".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Osma".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Ougr".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::LeftToRightTopToBottom,
                    ScriptDirection::TopToBottomRightToLeft,
                ],
                historic: true,
            },
        );
        data.insert(
            "Palm".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Pauc".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Perm".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Phag".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::LeftToRightTopToBottom],
                historic: true,
            },
        );
        data.insert(
            "Phli".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Phlp".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Phlv".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Phnx".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Plrd".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Prti".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Rjng".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Rohg".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Runr".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::TopToBottomRightToLeft, // Boustrophedon
                ],
                historic: true,
            },
        );
        data.insert(
            "Samr".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Sarb".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: true,
            },
        );
        data.insert(
            "Saur".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sgnw".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::TopToBottomLeftToRight,
                    ScriptDirection::LeftToRightTopToBottom,
                ],
                historic: false,
            },
        );
        data.insert(
            "Shaw".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Shrd".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sidd".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Shrd".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sind".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sinh".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sogd".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![
                    ScriptDirection::LeftToRightTopToBottom,
                    ScriptDirection::TopToBottomRightToLeft,
                ],
                historic: true,
            },
        );
        data.insert(
            "Sogo".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![
                    ScriptDirection::LeftToRightTopToBottom,
                    ScriptDirection::TopToBottomRightToLeft,
                ],
                historic: true,
            },
        );
        data.insert(
            "Sora".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Soyo".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Sund".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sunu".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Sylo".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Syrc".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Syre".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Syrj".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Syrn".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Tagb".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Takr".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tale".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Talu".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Taml".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tang".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: true,
            },
        );
        data.insert(
            "Tavt".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tayo".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: true,
            },
        );
        data.insert(
            "Telu".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tfng".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![
                    ScriptDirection::RightToLeftBottomToTop,
                    ScriptDirection::RightToLeftTopToBottom,
                    ScriptDirection::BottomToTopRightToLeft,
                    ScriptDirection::TopToBottomRightToLeft,
                    ScriptDirection::TopToBottomLeftToRight,
                ],
                historic: false,
            },
        );
        data.insert(
            "Tglg".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Thaa".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Thai".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tibt".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tirh".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tnsa".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Todr".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Toto".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Tutg".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Ugar".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Vaii".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Vith".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Wara".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Wcho".to_string(),
            ScriptData {
                category: Category::Alphabetic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: false,
            },
        );
        data.insert(
            "Xpeo".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Xsux".to_string(),
            ScriptData {
                category: Category::Logographic,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        data.insert(
            "Yezi".to_string(),
            ScriptData {
                category: Category::Abjad,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Yezi".to_string(),
            ScriptData {
                category: Category::Syllabic,
                directions: vec![ScriptDirection::TopToBottomRightToLeft],
                historic: false,
            },
        );
        data.insert(
            "Zand".to_string(),
            ScriptData {
                category: Category::Abugida,
                directions: vec![ScriptDirection::TopToBottomLeftToRight],
                historic: true,
            },
        );
        DataProvider { data }
    }

    pub fn get(&self, script: &str) -> Option<&ScriptData> {
        self.data.get(script)
    }
}
