pub use super::prelude::*;

pub fn collect_paths(
    dir: &Path,
    allowed_exts: &[String],
    variants: &mut Vec<(proc_macro2::Ident, String)>,
    current_rel_path: &str,
    logical_prefix: &str,
    seen: &mut HashSet<String>,
) {
    fn has_allowed_extension(file_name: &str, allowed_exts: &[String]) -> bool {
        allowed_exts
            .iter()
            .any(|ext| file_name.ends_with(&format!(".{ext}")))
    }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        let rel_path = if current_rel_path.is_empty() {
            name.to_string()
        } else {
            format!("{current_rel_path}/{name}")
        };

        if path.is_dir() {
            // logical path for the directory itself
            let logical_dir_path = if logical_prefix.is_empty() {
                rel_path.clone()
            } else {
                format!("{logical_prefix}/{rel_path}")
            };

            if !seen.contains(&logical_dir_path) {
                let ident_str = logical_dir_path.to_valid_rust_ident_with_no();
                let dir_ident = format_ident!("{}", ident_str);
                variants.push((dir_ident, logical_dir_path.clone()));
                seen.insert(logical_dir_path.clone());
            }

            // recurse into directory
            collect_paths(
                &path,
                allowed_exts,
                variants,
                &rel_path,
                logical_prefix,
                seen,
            );
        } else if path.is_file() && has_allowed_extension(&name, allowed_exts) {
            let logical_path = if logical_prefix.is_empty() {
                rel_path.clone()
            } else {
                format!("{logical_prefix}/{rel_path}")
            };

            if !seen.contains(&logical_path) {
                let ident_str = logical_path.to_valid_rust_ident_with_no();
                let variant_ident = format_ident!("{}", ident_str);
                variants.push((variant_ident, logical_path.clone()));
                seen.insert(logical_path);
            }
        }
    }
}
