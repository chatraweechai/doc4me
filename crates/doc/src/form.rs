//! Legacy form fields (Developer › Legacy Tools; ECMA-376 §17.16.17 `w:ffData`): `FORMTEXT`,
//! `FORMCHECKBOX` and `FORMDROPDOWN` fields with what each asks for — its kind, default, current
//! value, length and format limits, bookmark name and help text.
//!
//! A form field is an ordinary [`crate::InlineObject::Field`] whose `form` holds this data. A
//! text field's value is the field's result (plain text, five en spaces when blank, as Word
//! writes it); a drop-down's result is kept equal to its chosen entry so plain text, find and
//! export see it; a check box has no result (Word writes none) and is drawn from its state.

use serde::{Deserialize, Serialize};

/// Longest bookmark name Word gives a form field.
pub const MAX_NAME: usize = 20;
/// Longest help text Word keeps (F1 help); status bar text is shorter.
pub const MAX_HELP: usize = 255;
pub const MAX_STATUS: usize = 138;
/// Most entries, and the longest entry, of a drop-down form field.
pub const MAX_ENTRIES: usize = 25;
pub const MAX_ENTRY: usize = 50;
/// Longest default text, format and macro name kept.
pub const MAX_TEXT: usize = 255;
/// What a blank text form field shows: five en spaces.
pub const BLANK_TEXT: &str = "\u{2002}\u{2002}\u{2002}\u{2002}\u{2002}";

fn yes() -> bool {
    true
}
fn is_true(b: &bool) -> bool {
    *b
}

/// A form field's data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormField {
    /// Its bookmark name (`w:name`); empty for none.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Fill-in enabled (`w:enabled`): a disabled field can't be changed when the form is filled in.
    #[serde(default = "yes", skip_serializing_if = "is_true")]
    pub enabled: bool,
    /// Calculate on exit (`w:calcOnExit`): fields are updated when this one is left.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub calc_on_exit: bool,
    /// Macros run on entry and exit: kept for the file, never run.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub entry_macro: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub exit_macro: String,
    /// F1 help (`w:helpText`) and status bar text (`w:statusText`).
    #[serde(default, skip_serializing_if = "FormHelp::is_empty")]
    pub help: FormHelp,
    #[serde(default, skip_serializing_if = "FormHelp::is_empty")]
    pub status: FormHelp,
    pub kind: FormKind,
}

/// Help text: typed (`text`) or the name of an AutoText entry (`auto_text`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormHelp {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub auto_text: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
}

impl FormHelp {
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

/// The kind of form field and its own settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum FormKind {
    /// `FORMTEXT` (`w:textInput`).
    Text {
        #[serde(default)]
        input: TextType,
        /// Default text (for a calculation, the expression).
        #[serde(default, skip_serializing_if = "String::is_empty")]
        default: String,
        /// Most characters (0: unlimited).
        #[serde(default)]
        max_len: u16,
        /// Number picture (`0.00`), date picture (`M/d/yyyy`) or text case (`Uppercase`,
        /// `Lowercase`, `First capital`, `Title case`).
        #[serde(default, skip_serializing_if = "String::is_empty")]
        format: String,
    },
    /// `FORMCHECKBOX` (`w:checkBox`).
    CheckBox {
        /// Box size in points (`None`: as big as the text).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<f32>,
        #[serde(default)]
        default: bool,
        /// Its state when it differs from the default's (`w:checked`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        checked: Option<bool>,
    },
    /// `FORMDROPDOWN` (`w:ddList`).
    DropDown {
        #[serde(default)]
        entries: Vec<String>,
        /// Index of the default entry.
        #[serde(default)]
        default: u16,
        /// Index of the chosen entry when it isn't the default (`w:result`).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        result: Option<u16>,
    },
}

/// What a text form field takes (`w:type`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextType {
    #[default]
    Regular,
    Number,
    Date,
    CurrentDate,
    CurrentTime,
    Calculated,
}

impl TextType {
    pub const ALL: [TextType; 6] =
        [TextType::Regular, TextType::Number, TextType::Date, TextType::CurrentDate, TextType::CurrentTime, TextType::Calculated];
    /// The `w:type` value (`None` for regular text, which Word leaves out).
    pub fn ooxml(self) -> Option<&'static str> {
        match self {
            TextType::Regular => None,
            TextType::Number => Some("number"),
            TextType::Date => Some("date"),
            TextType::CurrentDate => Some("currentDate"),
            TextType::CurrentTime => Some("currentTime"),
            TextType::Calculated => Some("calculated"),
        }
    }
    pub fn from_ooxml(v: &str) -> TextType {
        match v {
            "number" => TextType::Number,
            "date" => TextType::Date,
            "currentDate" => TextType::CurrentDate,
            "currentTime" => TextType::CurrentTime,
            "calculated" => TextType::Calculated,
            _ => TextType::Regular,
        }
    }
    /// The name in commands and the Options dialog.
    pub fn name(self) -> &'static str {
        match self {
            TextType::Regular => "regular",
            TextType::Number => "number",
            TextType::Date => "date",
            TextType::CurrentDate => "currentDate",
            TextType::CurrentTime => "currentTime",
            TextType::Calculated => "calculated",
        }
    }
    pub fn from_name(v: &str) -> Option<TextType> {
        TextType::ALL.into_iter().find(|t| t.name() == v)
    }
    /// The Options dialog's label.
    pub fn label(self) -> &'static str {
        match self {
            TextType::Regular => "Regular text",
            TextType::Number => "Number",
            TextType::Date => "Date",
            TextType::CurrentDate => "Current date",
            TextType::CurrentTime => "Current time",
            TextType::Calculated => "Calculation",
        }
    }
    /// Whoever fills in the form types this kind (current date/time and calculations fill
    /// themselves).
    pub fn fillable(self) -> bool {
        matches!(self, TextType::Regular | TextType::Number | TextType::Date)
    }
}

impl FormField {
    pub fn new(kind: FormKind) -> FormField {
        FormField {
            name: String::new(),
            enabled: true,
            calc_on_exit: false,
            entry_macro: String::new(),
            exit_macro: String::new(),
            help: FormHelp::default(),
            status: FormHelp::default(),
            kind,
        }
    }
    pub fn text() -> FormField {
        FormField::new(FormKind::Text { input: TextType::Regular, default: String::new(), max_len: 0, format: String::new() })
    }
    pub fn check_box() -> FormField {
        FormField::new(FormKind::CheckBox { size: None, default: false, checked: None })
    }
    pub fn drop_down() -> FormField {
        FormField::new(FormKind::DropDown { entries: Vec::new(), default: 0, result: None })
    }
    /// The field code word for this kind.
    pub fn instr(&self) -> &'static str {
        match self.kind {
            FormKind::Text { .. } => "FORMTEXT",
            FormKind::CheckBox { .. } => "FORMCHECKBOX",
            FormKind::DropDown { .. } => "FORMDROPDOWN",
        }
    }
    /// The kind's name in commands.
    pub fn kind_name(&self) -> &'static str {
        match self.kind {
            FormKind::Text { .. } => "text",
            FormKind::CheckBox { .. } => "checkBox",
            FormKind::DropDown { .. } => "dropDown",
        }
    }
    /// Whether a check box is ticked.
    pub fn checked(&self) -> Option<bool> {
        match &self.kind {
            FormKind::CheckBox { default, checked, .. } => Some(checked.unwrap_or(*default)),
            _ => None,
        }
    }
    /// A drop-down's chosen entry index (the default's when none was chosen), within its entries.
    pub fn chosen(&self) -> Option<usize> {
        match &self.kind {
            FormKind::DropDown { entries, default, result } if !entries.is_empty() => {
                Some(usize::from(result.unwrap_or(*default)).min(entries.len() - 1))
            }
            _ => None,
        }
    }
    /// A drop-down's chosen entry text ("" without entries).
    pub fn chosen_text(&self) -> &str {
        match (&self.kind, self.chosen()) {
            (FormKind::DropDown { entries, .. }, Some(i)) => &entries[i],
            _ => "",
        }
    }
    /// What the field shows given its stored `result`: the box for a check box, the chosen
    /// entry for a drop-down (five en spaces with no entries, as Word), the result for text.
    pub fn display(&self, result: &str) -> String {
        match &self.kind {
            FormKind::CheckBox { .. } => if self.checked() == Some(true) { "\u{2612}" } else { "\u{2610}" }.to_string(),
            FormKind::DropDown { .. } if self.chosen().is_none() => BLANK_TEXT.to_string(),
            FormKind::DropDown { .. } => self.chosen_text().to_string(),
            FormKind::Text { .. } if result.is_empty() => BLANK_TEXT.to_string(),
            FormKind::Text { .. } => result.to_string(),
        }
    }
    /// The result a field resets to: the default text (blank: five en spaces), the default
    /// entry, or none for a check box.
    pub fn reset(&mut self) -> String {
        match &mut self.kind {
            FormKind::Text { default, .. } if default.is_empty() => return BLANK_TEXT.to_string(),
            FormKind::Text { default, .. } => return default.clone(),
            FormKind::CheckBox { checked, .. } => {
                *checked = None;
                return String::new();
            }
            FormKind::DropDown { result, .. } => *result = None,
        }
        self.chosen_text().to_string()
    }
    /// Bring values read from a file or given by a command within Word's limits.
    pub fn clamp(&mut self) {
        let cut = |s: &mut String, n: usize| {
            if s.chars().count() > n || s.chars().any(|c| c.is_control() || c == crate::para::OBJ) {
                *s = s.chars().filter(|c| !c.is_control() && *c != crate::para::OBJ).take(n).collect();
            }
        };
        cut(&mut self.name, MAX_NAME);
        cut(&mut self.entry_macro, MAX_TEXT);
        cut(&mut self.exit_macro, MAX_TEXT);
        cut(&mut self.help.text, MAX_HELP);
        cut(&mut self.status.text, MAX_STATUS);
        match &mut self.kind {
            FormKind::Text { default, format, max_len, .. } => {
                cut(default, MAX_TEXT);
                cut(format, MAX_TEXT);
                if *max_len > 0 && default.chars().count() > usize::from(*max_len) {
                    *default = default.chars().take(usize::from(*max_len)).collect();
                }
            }
            FormKind::CheckBox { size, .. } => {
                if let Some(s) = size {
                    *s = if s.is_finite() { s.clamp(1.0, 1638.0) } else { 10.0 };
                }
            }
            FormKind::DropDown { entries, default, result } => {
                entries.truncate(MAX_ENTRIES);
                for e in entries.iter_mut() {
                    cut(e, MAX_ENTRY);
                }
                let last = entries.len().saturating_sub(1) as u16;
                *default = (*default).min(last);
                if let Some(r) = result {
                    *r = (*r).min(last);
                }
            }
        }
    }
}

/// Apply a text form field's case format (`Uppercase`, `Lowercase`, `First capital`, `Title
/// case`) to `text`; other formats leave it alone.
pub fn apply_text_case(format: &str, text: &str) -> String {
    match format.to_ascii_lowercase().as_str() {
        "uppercase" => text.to_uppercase(),
        "lowercase" => text.to_lowercase(),
        "first capital" => {
            let mut c = text.chars();
            match c.next() {
                Some(f) => f.to_uppercase().chain(c).collect(),
                None => String::new(),
            }
        }
        "title case" => {
            let mut out = String::with_capacity(text.len());
            let mut start = true;
            for ch in text.chars() {
                if start && ch.is_alphabetic() {
                    out.extend(ch.to_uppercase());
                } else {
                    out.push(ch);
                }
                start = ch.is_whitespace();
            }
            out
        }
        _ => text.to_string(),
    }
}

/// A number in a Word number picture (`0`, `0.00`, `#,##0`, `#,##0.00`, `$#,##0.00;($#,##0.00)`,
/// `0%`, `0.00%`): digits `0` (always shown) and `#` (shown when needed), a `,` for thousands,
/// a `.` before the decimals, a `%` that shows hundredths, other characters as they are, and an
/// optional second section after `;` for negative numbers.
pub fn format_number(v: f64, picture: &str) -> String {
    let v = if v.is_finite() { v } else { 0.0 };
    let (pic, value, minus) = match picture.split_once(';') {
        Some((_, neg)) if v < 0.0 => (neg.split(';').next().unwrap_or(neg), -v, false),
        Some((pos, _)) => (pos, v.abs(), v < 0.0),
        None => (picture, v.abs(), v < 0.0),
    };
    let chars: Vec<char> = pic.chars().collect();
    let Some(first) = chars.iter().position(|c| matches!(c, '0' | '#')) else { return format!("{}{pic}", if minus { "-" } else { "" }) };
    let last = chars.iter().rposition(|c| matches!(c, '0' | '#' | ',' | '.')).unwrap_or(first).max(first);
    let prefix: String = chars[..first].iter().collect();
    let core: String = chars[first..=last].iter().collect();
    let suffix: String = chars[last + 1..].iter().collect();
    let value = if pic.contains('%') { value * 100.0 } else { value };
    let (int_pic, frac_pic) = core.split_once('.').unwrap_or((&core, ""));
    let decimals = frac_pic.chars().filter(|c| matches!(c, '0' | '#')).count().min(15);
    let min_decimals = frac_pic.chars().filter(|c| *c == '0').count().min(decimals);
    let min_int = int_pic.chars().filter(|c| *c == '0').count();
    // Halves round away from zero, as Word does (formatting alone rounds them to even).
    let scale = 10f64.powi(decimals as i32);
    let value = (value * scale).round() / scale;
    let fixed = format!("{value:.decimals$}");
    let (int_digits, frac_digits) = fixed.split_once('.').unwrap_or((&fixed, ""));
    let mut frac = frac_digits.to_string();
    while frac.len() > min_decimals && frac.ends_with('0') {
        frac.pop();
    }
    let mut int = int_digits.trim_start_matches('0').to_string();
    while int.len() < min_int {
        int.insert(0, '0');
    }
    if int_pic.contains(',') {
        let digits: Vec<char> = int.chars().collect();
        let mut grouped = String::new();
        for (i, c) in digits.iter().enumerate() {
            if i > 0 && (digits.len() - i).is_multiple_of(3) {
                grouped.push(',');
            }
            grouped.push(*c);
        }
        int = grouped;
    }
    let minus = minus && (int.chars().any(|c| c.is_ascii_digit() && c != '0') || frac.chars().any(|c| c != '0'));
    format!("{}{prefix}{int}{}{frac}{suffix}", if minus { "-" } else { "" }, if frac.is_empty() { "" } else { "." })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_pictures() {
        assert_eq!(format_number(1234.5, "#,##0.00"), "1,234.50");
        assert_eq!(format_number(1234.5, "0"), "1235");
        assert_eq!(format_number(-12.0, "$#,##0.00;($#,##0.00)"), "($12.00)");
        assert_eq!(format_number(12.0, "$#,##0.00;($#,##0.00)"), "$12.00");
        assert_eq!(format_number(-3.25, "0.00"), "-3.25");
        assert_eq!(format_number(0.125, "0.00%"), "12.50%");
        assert_eq!(format_number(0.5, "#.##"), ".5");
        assert_eq!(format_number(1234567.0, "#,##0"), "1,234,567");
        assert_eq!(format_number(f64::NAN, "0"), "0");
        assert_eq!(format_number(-0.001, "0.00"), "0.00");
    }

    #[test]
    fn display_reset_and_limits() {
        let mut t = FormField::text();
        assert_eq!(t.display(""), BLANK_TEXT);
        assert_eq!(t.display("Ada"), "Ada");
        let mut c = FormField::check_box();
        assert_eq!(c.display(""), "\u{2610}");
        if let FormKind::CheckBox { checked, .. } = &mut c.kind {
            *checked = Some(true);
        }
        assert_eq!((c.checked(), c.display("").as_str()), (Some(true), "\u{2612}"));
        assert_eq!(c.reset(), "");
        assert_eq!(c.checked(), Some(false));
        let mut d = FormField::new(FormKind::DropDown { entries: vec!["A".into(), "B".into()], default: 1, result: Some(9) });
        d.clamp();
        assert_eq!((d.chosen(), d.chosen_text()), (Some(1), "B"));
        assert_eq!(FormField::drop_down().display(""), BLANK_TEXT);
        // Over-long names, entries and defaults are cut to Word's limits.
        t.name = "x".repeat(40);
        t.kind = FormKind::Text { input: TextType::Number, default: "123456".into(), max_len: 3, format: "0".into() };
        t.clamp();
        assert_eq!(t.name.len(), MAX_NAME);
        assert!(matches!(&t.kind, FormKind::Text { default, .. } if default == "123"));
        assert_eq!(t.reset(), "123");
        let mut d =
            FormField::new(FormKind::DropDown { entries: (0..40).map(|i| format!("{i}{}", "y".repeat(80))).collect(), default: 30, result: None });
        d.clamp();
        assert!(
            matches!(&d.kind, FormKind::DropDown { entries, default: 24, .. } if entries.len() == MAX_ENTRIES && entries[0].chars().count() == MAX_ENTRY)
        );
    }

    #[test]
    fn text_cases() {
        assert_eq!(apply_text_case("Uppercase", "ada lovelace"), "ADA LOVELACE");
        assert_eq!(apply_text_case("lowercase", "ADA"), "ada");
        assert_eq!(apply_text_case("First capital", "ada lovelace"), "Ada lovelace");
        assert_eq!(apply_text_case("Title case", "ada  lovelace-byron"), "Ada  Lovelace-byron");
        assert_eq!(apply_text_case("0.00", "x"), "x");
    }
}
