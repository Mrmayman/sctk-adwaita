#[derive(Debug)]
pub(crate) struct FontPreference {
    pub pt_size: f32,
}

impl Default for FontPreference {
    fn default() -> Self {
        Self { pt_size: 10.0 }
    }
}

impl FontPreference {
    /// Parse config string like `Cantarell 12`, `Cantarell Bold 11`, `Noto Serif CJK HK Bold 12`.
    pub fn from_name_style_size(conf: &str) -> Option<Self> {
        // assume last is size, 2nd last is style and the rest is name.
        match conf.rsplit_once(' ') {
            Some((head, tail)) if tail.chars().all(|c| c.is_numeric()) => {
                let pt_size: f32 = tail.parse().unwrap_or(10.0);
                match head.rsplit_once(' ') {
                    Some((name, style)) if !name.is_empty() => Some(Self { pt_size }),
                    None if !head.is_empty() => Some(Self { pt_size }),
                    _ => None,
                }
            }
            Some((head, _)) if !head.is_empty() => Some(Self { pt_size: 10.0 }),
            None if !conf.is_empty() => Some(Self { pt_size: 10.0 }),
            _ => None,
        }
    }
}

#[test]
fn pref_from_multi_name_variant_size() {
    let pref = FontPreference::from_name_style_size("Noto Serif CJK HK Bold 12").unwrap();
    assert!((pref.pt_size - 12.0).abs() < f32::EPSILON);
}

#[test]
fn pref_from_name_variant_size() {
    let pref = FontPreference::from_name_style_size("Cantarell Bold 12").unwrap();
    assert!((pref.pt_size - 12.0).abs() < f32::EPSILON);
}

#[test]
fn pref_from_name_size() {
    let pref = FontPreference::from_name_style_size("Cantarell 12").unwrap();
    assert!((pref.pt_size - 12.0).abs() < f32::EPSILON);
}

#[test]
fn pref_from_name() {
    let pref = FontPreference::from_name_style_size("Cantarell").unwrap();
    assert!((pref.pt_size - 10.0).abs() < f32::EPSILON);
}
#[test]
fn pref_from_multi_name_style() {
    let pref = FontPreference::from_name_style_size("Foo Bar Baz Bold").unwrap();
    assert!((pref.pt_size - 10.0).abs() < f32::EPSILON);
}
