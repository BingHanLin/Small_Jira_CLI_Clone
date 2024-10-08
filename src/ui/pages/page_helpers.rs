use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    let text_len = text.len();

    let test = text_len.cmp(&width);

    match text_len.cmp(&width) {
        std::cmp::Ordering::Equal => text.to_owned(),
        std::cmp::Ordering::Less => {
            let right_space = width - text_len;

            let mut result = text.to_owned();

            result.push_str(&" ".repeat(right_space));

            result
        }
        std::cmp::Ordering::Greater => {
            if width == 0 {
                return "".to_owned();
            } else if width == 1 {
                return ".".to_owned();
            } else if width == 2 {
                return "..".to_owned();
            } else if width == 3 {
                return "...".to_owned();
            } else {
                let result = text.truncate_ellipse(width - 3);
                result.to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
