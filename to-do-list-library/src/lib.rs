#[cfg(test)]
mod tests {
    mod to_do_item {
        use super::super::ToDoItem;

        #[test]
        fn default_should_create_new_to_do_item() {
            let to_do_item = ToDoItem::default();

            let _expected = "Edit this to do item.".to_owned();
            let _actual = to_do_item.get_title();
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
}
