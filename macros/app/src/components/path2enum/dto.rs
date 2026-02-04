pub trait ToValidIdent: AsRef<str> {
    fn to_valid_rust_ident_with_no(&self) -> String {
        let s: &str = self.as_ref();

        // Split by logical path segments
        let parts: Vec<&str> = s.split('/').collect();
        let mut segments: Vec<String> = Vec::with_capacity(parts.len());

        for part in parts {
            // keep intended replacements: '&' -> And, '.' -> ・ (middle dot)
            let replaced = part.replace('&', "And").replace('.', "・");

            // split into words on common separators (safer closure)
            let words = replaced
                .split(['-', '_', ' '])
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            // PascalCase each part (works with Unicode letters)
            let mut pascal = String::new();
            for word in words {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    // use to_uppercase() for Unicode correctness
                    let first_up = first.to_uppercase().to_string();
                    pascal.push_str(&first_up);
                    pascal.push_str(chars.as_str());
                }
            }

            // Ensure segment starts with valid identifier char
            let safe = match pascal.chars().next() {
                Some('_') => pascal,
                Some(c) if c.is_alphabetic() => pascal,
                Some(c) if c.is_ascii_digit() => format!("_{pascal}"),
                Some(_) => format!("_{pascal}"),
                None => "_".to_string(),
            };

            segments.push(safe);
        }

        // Join segments with ノ to match your display style
        let joined = segments.join("ノ");

        // Final guard: make sure the overall first char is valid
        match joined.chars().next() {
            Some('_') => joined,
            Some(c) if c.is_alphabetic() => joined,
            Some(c) if c.is_ascii_digit() => format!("_{joined}"),
            Some(_) => format!("_{joined}"),
            None => "_".to_string(),
        }
    }
}

// Implement for str (works because str: AsRef<str>)
impl ToValidIdent for str {}
