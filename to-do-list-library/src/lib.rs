#[cfg(test)]
mod tests {
    mod to_do_item {

        mod default_should {
            use super::super::super::ToDoItem;

            #[test]
            fn create_new_to_do_item() {
                let to_do_item = ToDoItem::default();

                let _expected = "Edit this to do item.".to_owned();
                let _actual = to_do_item.get_title();
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
    }
}

pub struct ToDoItem {}

impl ToDoItem {
    pub fn default() -> ToDoItem {
        ToDoItem {}
    }

    pub fn get_title(&self) -> String {
        "Edit this to do item.".to_owned()
    }

    pub fn new(_title: &str) -> ToDoItem {
        ToDoItem {}
    }
}
