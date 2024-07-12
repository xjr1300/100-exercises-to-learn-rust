use bonus::data::{Status, TicketDraft, TicketPatch};
use bonus::{launch, CommandError};
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn it_works() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);

    let patch = TicketPatch {
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::InProgress),
        version: 0,
    };
    client.update(patch).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket.id, ticket_id);
    assert_eq!(ticket.status, Status::InProgress);
    assert_eq!(ticket.version, 1);
}

#[test]
fn version_is_not_match() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone()).unwrap();

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
    assert_eq!(ticket.version, 0);

    let patch = TicketPatch {
        id: ticket_id,
        title: None,
        description: None,
        status: Some(Status::InProgress),
        version: 1,
    };
    let result = client.update(patch);
    assert_eq!(Err(CommandError::VersionNotMatch), result);

    let ticket = client.get(ticket_id).unwrap().unwrap();
    assert_eq!(ticket_id, ticket.id);
    assert_eq!(ticket.status, Status::ToDo);
    assert_eq!(ticket.title, draft.title);
    assert_eq!(ticket.description, draft.description);
    assert_eq!(ticket.version, 0);
}
