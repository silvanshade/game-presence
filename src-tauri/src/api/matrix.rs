fn example() {
    use matrix_sdk::StateChanges;
    use ruma::{
        events::presence::{PresenceEvent, PresenceEventContent},
        presence::PresenceState,
        serde::Raw,
    };
    let sync_token = String::from("");
    let mut state_changes = StateChanges::new(sync_token);
    let event = {
        let content = {
            let state = PresenceState::Online;
            let mut value = PresenceEventContent::new(state);
            value.status_msg = Some(String::from("Playing Steam"));
            value
        };
        let sender = ruma::user_id!("@someone:matrix.org").to_owned();
        PresenceEvent { content, sender }
    };
    let json = r#""#;
    let raw_event = serde_json::from_str::<Raw<PresenceEvent>>(json).unwrap();
    state_changes.add_presence_event(event, raw_event);
}