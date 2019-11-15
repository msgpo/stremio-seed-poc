use seed::{prelude::*, *};

#[derive(Clone, Debug)]
pub struct Group<T> {
    pub id: GroupId,
    pub label: Option<String>,
    pub items: Vec<GroupItem<T>>,
    pub limit: usize,
    pub required: bool,
}

#[derive(Clone, Debug)]
pub struct GroupItem<T> {
    pub id: GroupItemId,
    pub label: String,
    pub value: T,
    pub selected: bool,
}

// ------ ------
//     Model
// ------ ------

pub type GroupId = String;
pub type GroupItemId = String;

pub struct Model {
}

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model {
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
pub enum Msg {
    ItemClicked(GroupId, GroupItemId)
}

pub fn update<T: 'static, ParentMsg>(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, mut groups: Vec<Group<T>>, on_change: impl FnOnce(Vec<Group<T>>) -> ParentMsg) -> Option<ParentMsg> {
    match msg {
        // @TODO: Refactor + comments
        Msg::ItemClicked(group_id, item_id) => {
            let first_selected_address = groups.iter().enumerate().find_map(|(group_index, group)| {
                if group.id == group_id {
                    if let Some(item_index) = group.items.iter().position(|item| item.selected) {
                        Some((group_index, item_index))
                    } else {
                        None
                    }
                } else {
                    None
                }
            });

            let selected_count: usize = groups.iter().map(|group| {
                group.items.iter().filter(|item| item.selected).count()
            }).sum();

            let group = groups.iter_mut().find(|group| {
                group.id == group_id && group.items.iter().any(|item| item.id == item_id)
            }).expect("`Group` with given `group_id`, which contains `GroupItem` with given `item_id`");

            let item = group.items.iter_mut().find(|item| item.id == item_id)
                .expect("`GroupItem` with given `item_id`");

            let changed = if item.selected {
                if !group.required || selected_count > 1 {
                    item.selected = false;
                    true
                } else {
                    false
                }
            } else {
                if selected_count < group.limit {
                    item.selected = true;
                    true
                } else {
                    if let Some((first_selected_group_index, first_selected_item_index)) = first_selected_address {
                        item.selected = true;
                        groups
                            .get_mut(first_selected_group_index)
                            .unwrap()
                            .items
                            .get_mut(first_selected_item_index)
                            .unwrap().selected = false;
                        true
                    } else {
                        false
                    }
                }
            };

            if changed {
                let groups_with_selected_items = groups.into_iter().filter_map(|mut group| {
                    group.items = group.items.into_iter().filter(|item| item.selected).collect();
                    if group.items.is_empty() {
                        None
                    } else {
                        Some(group)
                    }
                }).collect::<Vec<_>>();
                Some(on_change(groups_with_selected_items))
            } else {
                None
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<T: Clone>(model: &Model, groups: &[Group<T>]) -> Node<Msg> {
    div![
        class!["multi-select"],
        style!{
            St::Float => "left",
            St::Margin => px(5),
        },
        groups.iter().map(view_group)
    ]
}

pub fn view_group<T: Clone>(group: &Group<T>) -> Node<Msg> {
    div![
        class!["group"],
        match &group.label {
            Some(label) => {
                div![
                    style!{
                        St::MarginLeft => px(5),
                        St::FontWeight => "bold",
                    },
                    label,
                ]
            },
            None => empty![],
        },
        group.items.iter().map(|item| view_group_item(&group.id, item))
    ]
}

pub fn view_group_item<T: Clone>(group_id: &str, item: &GroupItem<T>) -> Node<Msg> {
    div![
        class!["group-item"],
        style!{
            St::Background => if item.selected { "green" } else { "initial" }
        },
        simple_ev(Ev::Click, Msg::ItemClicked(group_id.to_owned(), item.id.clone())),
        item.label,
    ]
}


