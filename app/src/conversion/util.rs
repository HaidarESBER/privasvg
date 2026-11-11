use visioncortex::PathSimplifyMode;

pub fn path_simplify_mode(s: &str) -> PathSimplifyMode {
    match s {
        "polygon" => PathSimplifyMode::Polygon,
        "spline" => PathSimplifyMode::Spline,
        "none" => PathSimplifyMode::None,
        _ => panic!("unknown PathSimplifyMode {}", s),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_modes() {
        assert!(matches!(path_simplify_mode("polygon"), PathSimplifyMode::Polygon));
        assert!(matches!(path_simplify_mode("spline"), PathSimplifyMode::Spline));
        assert!(matches!(path_simplify_mode("none"), PathSimplifyMode::None));
    }

    #[test]
    #[should_panic(expected = "unknown PathSimplifyMode")]
    fn rejects_unknown_mode() {
        path_simplify_mode("bogus");
    }
}
