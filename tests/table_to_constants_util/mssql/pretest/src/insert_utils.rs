fn get_before_and_after(statement: &str) -> (String, String) {
    let (before, with_placeholder) = statement.split_once("(?").unwrap();
    let after = with_placeholder
        .split_once("?)")
        .map(|(_, after)| after)
        .unwrap_or("");
    (before.to_string(), after.to_string())
}

pub fn to_substituted(statement: &str, args: &[String]) -> String {
    let (before, after) = get_before_and_after(&statement);

    let substitutions = format!(
        "({})",
        args.into_iter()
            .enumerate()
            .map(|(index, arg)| if index == 0 {
                arg.to_string()
            } else {
                format!("'{}'", arg)
            })
            .collect::<Vec<String>>()
            .join(", ")
    );
    [before, substitutions, after].join("")
}
