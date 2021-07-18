/// Join vector of strings to a human-readable string with conjunction.
///
/// ```
/// use teljari::join_with_conj;
/// 
/// let list: Vec<&str> = vec!("Me", "Myself", "I");
/// let conj: &str = "and";
/// let result = join_with_conj(&list, &conj);
/// assert_eq!(result, "Me, Myself and I");
/// ```
pub fn join_with_conj(list: &[&str], conj: &str) -> String {
    if list.is_empty() { panic!("Array should not be empty"); }
    if list.len() == 1 { return String::from(list[0]); }

    let first_ones: String = list[0..list.len() - 1].join(", ");
    let last: &str = list.last().unwrap();

    return format!("{} {} {}", first_ones, conj, last);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Array should not be empty")]
    fn panics_on_empty_array() {
        let list: Vec<&str> = Vec::new();
        let conj: &str = "whatever";

        join_with_conj(&list, conj);
    }

    #[test]
    fn does_not_add_conjunction_to_one_member_list() {
        let list: Vec<&str> = vec!("Finnish");
        let conj: &str = "and (which should not be printed)";
        let expected = "Finnish";
        let result = join_with_conj(&list, conj);
        assert_eq!(result, expected);
    }

    #[test]
    fn does_not_add_commas_to_two_member_list() {
        let list: Vec<&str> = vec!("Finnish", "Karelian");
        let conj: &str = "or";
        let expected = "Finnish or Karelian";
        let result = join_with_conj(&list, conj);
        assert_eq!(result, expected);
    }

    #[test]
    fn joins_with_and_conjunction() {
        let list: Vec<&str> = vec!("Finnish", "Swedish", "Norwegian", "Danish");
        let conj: &str = "and";
        let expected = "Finnish, Swedish, Norwegian and Danish";
        let result = join_with_conj(&list, conj);
        assert_eq!(result, expected);
    }

    #[test]
    fn joins_with_or_conjunction() {
        let list: Vec<&str> = vec!("Finnish", "Swedish", "Norwegian", "Danish");
        let conj: &str = "or worse";
        let expected = "Finnish, Swedish, Norwegian or worse Danish";
        let result = join_with_conj(&list, conj);
        assert_eq!(result, expected);
    }
}
