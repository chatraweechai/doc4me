use super::*;

fn flagged(text: &str) -> Vec<(&str, Vec<String>)> {
    check(text).into_iter().map(|i| (text.get(i.start..i.end).unwrap_or(""), i.suggestions)).collect()
}

#[test]
fn common_misspellings_get_the_royal_society_spelling() {
    assert_eq!(flagged("ขออนุญาติลาป่วย"), [("อนุญาติ", vec!["อนุญาต".to_string()])]);
    assert_eq!(flagged("ผัดกระเพราไก่"), [("กระเพรา", vec!["กะเพรา".to_string()])]);
    assert_eq!(flagged("ส่งอีเมล์ถึงฉัน"), [("อีเมล์", vec!["อีเมล".to_string()])]);
    let i = &check("สังเกตุดูนะ")[0];
    assert_eq!((i.kind, i.lang), (IssueKind::Spelling, ProofLang::Th));
    assert!(i.message.contains("สังเกต"), "{}", i.message);
    // The correct spellings are left alone.
    for ok in ["ขออนุญาตลาป่วย", "ผัดกะเพราไก่", "ส่งอีเมลถึงฉัน", "สังเกตดูนะ", "ภาษาไทยเป็นภาษาที่สวยงาม"]
    {
        assert!(check(ok).is_empty(), "{ok}: {:?}", check(ok));
    }
}

/// The letters of a misspelling inside a longer correct word, or across two correct words, are
/// not marked: "กระ" (a hawksbill) followed by "เพราะ" (because) holds "กระเพรา".
#[test]
fn misspellings_are_marked_only_as_whole_words() {
    assert!(check("ตัวกระเพราะว่ายน้ำ").is_empty(), "{:?}", check("ตัวกระเพราะว่ายน้ำ"));
    assert!(misspelled("ตัวกระเพราะว่ายน้ำ").is_empty());
}

#[test]
fn characters_typed_out_of_order_are_marked_with_the_word_fixed() {
    // เ typed twice for แ.
    assert_eq!(flagged("เเม่"), [("เเม่", vec!["แม่".to_string()])]);
    // ํ + า for ำ.
    assert_eq!(flagged("น\u{0E49}\u{0E4D}\u{0E32}"), [("น\u{0E49}\u{0E4D}\u{0E32}", vec!["น้ำ".to_string()])]);
    // A tone mark before the vowel above, and after ำ.
    assert_eq!(flagged("ก\u{0E48}\u{0E35}"), [("ก\u{0E48}\u{0E35}", vec!["กี่".to_string()])]);
    assert_eq!(flagged("น\u{0E33}\u{0E49}"), [("น\u{0E33}\u{0E49}", vec!["น้ำ".to_string()])]);
    // ์ before the vowel: สิทธิ์.
    assert_eq!(flagged("สิทธ\u{0E4C}\u{0E34}"), [("สิทธ\u{0E4C}\u{0E34}", vec!["สิทธิ์".to_string()])]);
    // The same mark twice.
    assert_eq!(flagged("ที่\u{0E48}"), [("ที่\u{0E48}", vec!["ที่".to_string()])]);
    // Two different tone marks: wrong, but which one was meant can't be told.
    assert_eq!(flagged("ก\u{0E48}\u{0E49}"), [("ก\u{0E48}\u{0E49}", Vec::<String>::new())]);
    // Correct text in every one of these shapes.
    for ok in ["แม่", "น้ำ", "กี่", "สิทธิ์", "ที่", "ก็", "ฤๅษี", "พุทฺธ", "ปั่น", "กุ้ง", "ปู่"]
    {
        assert!(orthography(ok).is_empty(), "{ok}: {:?}", orthography(ok));
    }
}

#[test]
fn suggestions_and_text_without_thai() {
    assert_eq!(suggest("อนุญาติ"), ["อนุญาต"]);
    assert_eq!(suggest("เเม่"), ["แม่"]);
    assert!(suggest("แม่").is_empty());
    assert!(check("Hello world").is_empty());
    assert!(check("").is_empty());
    // Words in the user dictionary are the user's call.
    crate::add_word("คลินิค");
    assert!(check("ไปคลินิคมา").is_empty());
    crate::remove_word("คลินิค");
    assert_eq!(flagged("ไปคลินิคมา"), [("คลินิค", vec!["คลินิก".to_string()])]);
}

#[test]
fn the_misspelling_list_is_well_formed() {
    let list = misspellings();
    assert!(list.len() >= 14);
    for (w, r) in list {
        assert!(w.chars().all(|c| is_thai(c) || c == ' '), "{w}");
        assert!(r.chars().all(|c| is_thai(c) || c == ' '), "{r}");
        assert!(orthography(r).is_empty(), "{r}: the correct form is typed in order");
        assert!(!list.iter().any(|(w2, _)| w2 == r), "{r} is itself listed as wrong");
    }
}

/// Hostile and odd input never panics: lone marks, marks at the start, very long text.
#[test]
fn odd_text_never_panics() {
    for t in ["\u{0E48}", "\u{0E48}\u{0E48}", "เ", "เเ", "\u{0E33}\u{0E49}", "a\u{0E40}\u{0E40}", " \u{0E4D}\u{0E32} "] {
        let _ = check(t);
        let _ = suggest(t);
    }
    let long = "ภาษาไทยอนุญาติ ".repeat(2000);
    assert_eq!(check(&long).len(), 2000);
}

/// A listed misspelling that is the start of a longer word the segmenter doesn't know is left
/// alone: คลินิคัล ("clinical") holds คลินิค, and "fixing" it would write คลินิกัล.
#[test]
fn a_misspelling_inside_a_longer_word_is_not_marked() {
    for ok in ["คลินิคัล", "เภสัชกรรมคลินิคัล", "การทดลองคลินิคัล"]
    {
        assert!(check(ok).is_empty(), "{ok}: {:?}", check(ok));
    }
    assert_eq!(flagged("ไปคลินิคมา"), [("คลินิค", vec!["คลินิก".to_string()])]);
}

/// Ignore All and Add to Dictionary clear typing-order marks too.
#[test]
fn words_in_the_user_dictionary_get_no_typing_order_mark() {
    crate::add_word("เเม่น้ำ");
    assert!(check("เเม่น้ำ").is_empty(), "{:?}", check("เเม่น้ำ"));
    assert!(!check("เเก").is_empty());
    crate::remove_word("เเม่น้ำ");
    assert!(!check("เเม่น้ำ").is_empty());
}

/// Every correct spelling in the list, written in a sentence, gets no mark (precision first),
/// and a wrong form standing as its own word is marked with its correction.
#[test]
fn every_listed_correction_is_left_alone_in_a_sentence() {
    for (wrong, right) in misspellings() {
        let ok = format!("เขาเขียนว่า {right} ในเอกสาร");
        assert!(check(&ok).is_empty(), "{right}: {:?}", check(&ok));
        let bad = format!("เขาเขียนว่า {wrong} ในเอกสาร");
        let issues = check(&bad);
        if let Some(i) = issues.first() {
            assert_eq!(bad.get(i.start..i.end), Some(*wrong), "{wrong}");
            assert_eq!(i.suggestions, [right.to_string()]);
        }
    }
}
