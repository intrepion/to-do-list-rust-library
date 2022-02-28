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

            #[test]
            fn create_new_to_do_item_not_done() {
                let to_do_item = ToDoItem::default();

                let expected = false;
                let actual = to_do_item.get_done();

                assert_eq!(actual, expected);
            }

            #[test]
            fn create_new_to_do_item_not_hidden() {
                let to_do_item = ToDoItem::default();

                let expected = false;
                let actual = to_do_item.get_hidden();

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
                let mut to_do_item = ToDoItem::default();
                to_do_item.set_done(true);

                let expected = true;
                let actual = to_do_item.get_done();

                assert_eq!(actual, expected);
            }
        }

        mod set_hidden_should {
            use super::super::super::ToDoItem;

            #[test]
            fn update_hidden() {
                let mut to_do_item = ToDoItem::default();
                to_do_item.set_hidden(true);

                let expected = true;
                let actual = to_do_item.get_hidden();

                assert_eq!(actual, expected);
            }
        }
    }
}

pub struct ToDoItem {
    done: bool,
    hidden: bool,
    title: String,
}

impl ToDoItem {
    pub fn default() -> ToDoItem {
        ToDoItem::new("Edit this to-do item.")
    }

    pub fn get_done(&self) -> bool {
        self.done
    }

    pub fn get_hidden(&self) -> bool {
        self.hidden
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn new(title: &str) -> ToDoItem {
        ToDoItem {
            done: false,
            hidden: false,
            title: title.to_owned(),
        }
    }

    pub fn set_done(&mut self, new_done: bool) {
        self.done = new_done;
    }

    pub fn set_hidden(&mut self, new_hidden: bool) {
        self.hidden = new_hidden;
    }

    pub fn set_title(&mut self, new_title: &str) {
        self.title = new_title.to_owned();
    }
}
