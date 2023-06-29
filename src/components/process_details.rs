use sysinfo::{Process, ProcessExt, SystemExt, UserExt};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
};

use crate::state::State;

pub fn process_details(state: &mut State) -> List {
    let process_index = state.selected_process.selected();
    let texts: Vec<String> = if let Some(process_index) = process_index {
        let processes: Vec<&Process> = state.system.processes().values().collect();
        let process = processes.get(process_index).copied();

        if let Some(process) = process {
            let name = process.name().to_owned();
            let cmd = process.cmd().join(" ");
            let cwd = process.cwd().display().to_string();
            let run_time = process.run_time();
            let uid = process.user_id();
            let status = process.status().to_string();

            let username = if let Some(uid) = uid {
                if let Some(user) = state.system.get_user_by_id(uid) {
                    user.name().to_owned()
                } else {
                    "SYSTEM".to_owned()
                }
            } else {
                "SYSTEM".to_owned()
            };

            vec![
                format!("Name: {}", name),
                "".to_owned(),
                format!("Command: {}", cmd),
                "".to_owned(),
                format!("Working Directory: {}", cwd),
                "".to_owned(),
                format!("Status: {}", status),
                "".to_owned(),
                format!("Run Time: {}", run_time),
                "".to_owned(),
                format!("User: {}", username),
            ]
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let spans = texts.iter().map(|text| {
        Spans::from(Span::styled(
            text.clone(),
            if text.is_empty() {
                Style::default()
                    .fg(Color::White)
            } else {
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            },
        ))
    });

    let items: Vec<ListItem> = spans.map(ListItem::new).collect();
    let component = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Process Details"),
    );

    component
}
