struct Item {
    id: u32,
    name: String,
}

fn create_item(id: u32, name: String) -> Item {
    Item { id, name }
}

fn read_item(target_id: u32, items: &[Item]) -> Option<&Item> {
    items.iter().find(|&item| item.id == target_id)
}

fn update_item(target_id: u32, items: &mut Vec<Item>, new_name: &str) -> bool {
    if let Some(item) = items.iter_mut().find(|item| item.id == target_id) {
        item.name = new_name.to_string();
        true
    } else {
        false
    }
}

fn delete_item(items: &mut Vec<Item>, target_id: u32) -> bool {
    if let Some(index) = items.iter().position(|item| item.id == target_id) {
        items.remove(index);
        true
    } else {
        false
    }
}

fn main() {
    println!("Hello, world!");
}
