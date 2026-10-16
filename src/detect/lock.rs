//! A tiny scan of `flake.lock` text: no JSON crate, no nix eval (detect C).

/// The lock has a node `name`: the quoted key followed by `:` and an
/// object. Input maps (`"name": "node"`) and follows lists
/// (`"name": [..]`) do not count.
///
/// ```
/// use pipelith::detect::has_node;
///
/// assert!(has_node(r#"{"nodes": {"hk": {}}}"#, "hk"));
/// assert!(!has_node(r#"{"inputs": {"hk": "hk_2"}}"#, "hk"));
/// ```
#[must_use]
pub fn has_node(lock: &str, name: &str) -> bool {
    let key = format!("\"{name}\"");
    lock.split(key.as_str()).skip(1).any(opens_object)
}

/// The text after a key is `: {`, blanks allowed around the colon.
fn opens_object(after: &str) -> bool {
    after
        .trim_start()
        .strip_prefix(':')
        .is_some_and(|value| value.trim_start().starts_with('{'))
}

#[cfg(test)]
mod tests;
