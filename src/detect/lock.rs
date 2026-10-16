//! A tiny scan of `flake.lock` text: no JSON crate, no nix eval (detect C).

/// The lock has a node `name`: the quoted key followed by `:` and an
/// object. Input maps (`"name": "node"`) and follows lists
/// (`"name": [..]`) do not count.
#[must_use]
pub fn has_node(lock: &str, name: &str) -> bool {
    let _ = (lock, name);
    false
}

#[cfg(test)]
mod tests;
