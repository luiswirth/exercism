pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    list.windows(2)
        .map(|win| format!("For want of a {0} the {1} was lost.", win[0], win[1]))
        .chain(std::iter::once(format!(
            "And all for the want of a {}.",
            list[0]
        )))
        .collect::<Vec<_>>()
        .join("\n")
}
