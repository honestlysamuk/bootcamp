use ellipse::Ellipse;

use crate::models::Epic;

pub fn get_column_string(text: &str, width: usize) -> String {
    let string = if text.len() <= width {
        text.to_owned()
    } else if text.len() > width && width > 3 {
        text.truncate_ellipse(width - 3).into_owned()
    } else {
        ".".repeat(width).to_owned()
    };
    format!("{string:<width$.*}", width)
}

pub fn format_jira_epic(epic_id: u32, epic: &Epic) -> String {
    let id_text = get_column_string(&format!("{}", &epic_id), 12);
    let name_text = get_column_string(&format!("{}", &epic.name), 34);
    let status_text = get_column_string(&format!("{:?}", &epic.status), 18);
    format!("{}|{}|{}", id_text, name_text, status_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetesq";

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
