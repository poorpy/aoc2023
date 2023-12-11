use super::*;

#[test]
fn test_array_expansion() {
    let input = include_str!("./_test_input.txt");
    let expected = include_str!("./_test_expected.txt");

    let expected = expected
        .lines()
        .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect_vec())
        .collect_vec();

    let expanded = expand(
        input
            .lines()
            .map(|l| l.chars().filter(|c| !c.is_whitespace()).collect_vec())
            .collect_vec(),
    );

    for (id, row) in expanded.into_iter().enumerate() {
        assert_eq!(row, expected[id], "row: {id} {row:?}")
    }
}
