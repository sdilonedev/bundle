use sysinfo::{CpuExt, SystemExt};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem},
};

use crate::state::State;

pub fn cpu_monitor(state: &mut State) -> List {
    let cpu_info = state.system.global_cpu_info();
    let brand = cpu_info.brand();
    let usage = cpu_info.cpu_usage();
    let cpus = state.system.cpus();

    let texts = [
        format!("Usage: {:.1}%", usage),
        format!("Brand: {}", brand),
        format!("Cores: {}", cpus.len()),
    ];

    let spans: Vec<Spans> = texts
        .iter()
        .map(|text| {
            Spans::from(Span::styled(
                text.clone(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let items: Vec<ListItem> = spans.into_iter().map(ListItem::new).collect();

    let component = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title("CPU Info"),
    );

    component
}
