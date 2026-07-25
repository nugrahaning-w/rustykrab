use crate::prelude::*;

#[test]
fn add_event() {
    let mut node = Node::new(NodeId::new("button"), NodeKind::Button);

    node.add_event(EventHandler::new(
        EventKind::Click,
        EventAction::new("login"),
    ));

    assert_eq!(node.event_count(), 1,);
}

#[test]
fn node_has_events() {
    let mut node = Node::new(NodeId::new("button"), NodeKind::Button);

    assert!(!node.has_events());

    node.add_event(EventHandler::new(
        EventKind::Click,
        EventAction::new("login"),
    ));

    assert!(node.has_events());
}

#[test]
fn preserve_event_order() {
    let mut node = Node::new(NodeId::new("button"), NodeKind::Button);

    node.add_event(EventHandler::new(
        EventKind::Click,
        EventAction::new("login"),
    ));

    node.add_event(EventHandler::new(
        EventKind::Submit,
        EventAction::new("submit"),
    ));

    let events = node.events().events();

    assert_eq!(events[0].action().name(), "login",);

    assert_eq!(events[1].action().name(), "submit",);
}
