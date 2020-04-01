use regex::Regex;

/// Structure representing the regular expresion rules to follow while parsing
pub struct Rules {
    pub occur_regex: Regex,
    pub value_regex: Regex,
    pub user_cmd_regex: Regex,
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            // =#(row1, col1, row2, col2, value)
            occur_regex: Regex::new(r"^=#\((\d+,){4}(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\)$").unwrap(),
            // number between 0 and 255
            value_regex: Regex::new(r"^(1?[0-9]{1,2})$|^(2[0-4][0-9])$|^(25[0-5])$").unwrap(),
            // row col =#(row1, col1, row2, col2, value)
            user_cmd_regex: Regex::new(r"^(\d+) (\d+) (=#\((\s*\d+\s*,){4}\s*(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5])\s*\)|(1?[0-9]{1,2}|2[0-4][0-9]|25[0-5]))$").unwrap(),
        }
    }
}
