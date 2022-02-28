#[cfg(test)]
mod tests {
    mod to_do_item {
        use super::super::ToDoItem;

        #[test]
        fn default_should_create_new_to_do_item() {
            let _to_do_item = ToDoItem::default();
        }
    }
}

pub struct ToDoItem {}

impl ToDoItem {
    pub fn default() -> ToDoItem {
        ToDoItem {}
    }
}
