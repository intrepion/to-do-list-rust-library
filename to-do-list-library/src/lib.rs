#[cfg(test)]
mod tests {
    mod to_do_item {

        mod default_should {
            use super::super::super::ToDoItem;

            #[test]
            fn create_new_to_do_item() {
                let to_do_item = ToDoItem::default();

                let expected = "Edit this to-do item.".to_owned();
                let actual = to_do_item.get_title();

                assert_eq!(actual, expected);
            }
        }

        mod new_should {
            use super::super::super::ToDoItem;

            #[test]
            fn create_new_to_do_item_with_custom_title() {
                let to_do_item = ToDoItem::new("Make a code library.");

                let expected = "Make a code library.".to_owned();
                let actual = to_do_item.get_title();

                assert_eq!(actual, expected);
            }
        }

        mod set_title_should {
            use super::super::super::ToDoItem;

            #[test]
            fn update_title() {
                let mut to_do_item = ToDoItem::new("Make a code libary.");
                to_do_item.set_title("Make a code library.");

                let expected = "Make a code library.".to_owned();
                let actual = to_do_item.get_title();

                assert_eq!(actual, expected);
            }
        }

        mod set_done_should {
            use super::super::super::ToDoItem;

            #[test]
            fn update_done() {
                let _to_do_item = ToDoItem::new("Make a code library.");
            }
        }
    }
}

pub struct ToDoItem {
    title: String,
}

impl ToDoItem {
    pub fn default() -> ToDoItem {
        ToDoItem::new("Edit this to-do item.")
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn new(title: &str) -> ToDoItem {
        ToDoItem {
            title: title.to_owned(),
        }
    }

    pub fn set_title(&mut self, new_title: &str) {
        self.title = new_title.to_owned();
    }
}
