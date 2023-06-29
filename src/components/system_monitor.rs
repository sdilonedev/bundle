use std::time::SystemTime;

use sysinfo::SystemExt;
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::state::State;

pub fn system_monitor(state: &mut State) -> List {
    // Misc
    let hostname = state.system.host_name().unwrap_or_else(|| "Unknown".to_string());

    // OS
    let os_name = state.system.distribution_id();
    let os_ver = state.system.os_version().unwrap_or_else(|| "unknown".to_string());
    let os = format!("{} {}", os_name, os_ver);

    // Uptime
    let start = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let uptime = start - state.system.boot_time();

    let uptime_h = uptime / 3600;
    let uptime_m = (uptime - (uptime_h * 3600)) / 60;
    let uptime_s = uptime - (uptime_h * 3600) - (uptime_m * 60);
    let uptime_format = format!("{:02}:{:02}:{:02}", uptime_h, uptime_m, uptime_s);

    let texts = [
        format!("Uptime: {}", uptime_format),
        format!("Hostname: {}", hostname),
        format!("OS: {}", os),
    ];

    let spans = texts.iter().map(|text| {
        Spans::from(Span::styled(
            text.clone(),
            Style::default().fg(Color::White),
        ))
    });

    let items: Vec<ListItem> = spans.map(|span| ListItem::new(span)).collect();
    let component = List::new(items).block(
        Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Plain)
            .title("System"),
    );
    component
}
