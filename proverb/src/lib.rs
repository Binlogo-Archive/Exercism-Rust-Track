use std::iter::once;
pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    if list.is_empty() {
        return String::new();
    }
    list.windows(2)
        .map(|slice| {
            format!(
                "For want of a {} the {} was lost.\n",
                slice.first().unwrap(),
                slice.last().unwrap()
            )
        })
        .chain(once(format!(
            "And all for the want of a {}.",
            list.first().unwrap()
        )))
        .collect()
}