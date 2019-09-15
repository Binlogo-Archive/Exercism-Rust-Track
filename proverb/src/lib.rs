pub fn build_proverb(list: &[&str]) -> String {
    // unimplemented!("build a proverb from this list of items: {:?}", list)
    let mut res = String::new();
    if list.is_empty() {
        return res;
    }
    for i in 0..list.len() - 1 {
        res = res + &format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
    }
    res + &format!("And all for the want of a {}.", list.first().unwrap())
}
