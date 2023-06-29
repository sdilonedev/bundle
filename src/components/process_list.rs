use sysinfo::{ProcessExt, SystemExt};
use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::state::State;

pub fn process_list(state: &mut State) -> Table {
    let selected_style = Style::default().bg(Color::White).fg(Color::Black);

    fn get_style_by_percent(value: f64, max: f64) -> Style {
        let percent = (value / max) * 100.0;

        if percent > 75.0 {
            Style::default().fg(Color::Red)
        } else if percent > 50.0 {
            Style::default().fg(Color::LightRed)
        } else if percent > 25.0 {
            Style::default().fg(Color::Yellow)
        } else if percent > 5.0 {
            Style::default().fg(Color::LightYellow)
        } else if percent > 1.0 {
            Style::default().fg(Color::LightGreen)
        } else if percent > 0.1 {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        }
    }

    let header_cells = ["Name", "PID", "Memory", "CPU", "Disk"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::LightBlue)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = state
        .system
        .processes()
        .iter()
        .map(|(pid, process)| {
            let name = process.name().to_owned();
            let ram_usage = process.memory() as f64 / 1024.0 / 1024.0;
            let cpu_usage = process.cpu_usage() / state.system.cpus().len() as f32;
            let disk_usage = (process.disk_usage().read_bytes + process.disk_usage().written_bytes)
                as f64
                / 1024.0
                / 1024.0;

            let data = [
                name,
                pid.to_string(),
                format!("{:.1} MB", ram_usage),
                format!("{:.1}%", cpu_usage),
                format!("{:.1} MB/S", disk_usage),
            ];

            let styles = [
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                Style::default().fg(Color::LightCyan),
                get_style_by_percent(ram_usage, state.system.total_memory() as f64 / 1024.0 / 1024.0),
                get_style_by_percent(cpu_usage as f64, 100.0),
                get_style_by_percent(disk_usage, 100.0),
            ];

            let cells = data
                .iter()
                .zip(styles.iter())
                .map(|(text, style)| Cell::from(text.clone()).style(*style));

            Row::new(cells).height(1)
        });

    let component = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Process List"))
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Percentage(44),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(7),
            Constraint::Percentage(19),
        ]);

    component
}
