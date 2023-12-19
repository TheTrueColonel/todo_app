use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::app::{App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    ui_main(f, app);

    if app.currently_adding {
        ui_adding(f, app);
    }
}

fn ui_main(f: &mut Frame, app: &App) {
    let chunks = Layout::new(Direction::Vertical, [
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ]).split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Todo List",
        Style::default().fg(Color::Green)
    )).block(title_block);

    f.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();

    let item_style = if app.todo_cursor_pos == -1 {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::Yellow)
    };

    list_items.push(ListItem::new(Line::from(Span::styled(
        "New Entry",
        item_style
    ))));

    for (i, item) in app.todo_items.iter().enumerate() {
        let mut item_style = if app.todo_cursor_pos == i as isize {
            Style::default().fg(Color::Black).bg(Color::White)
        } else {
            Style::default().fg(Color::Yellow)
        };

        if item.completed {
            item_style = item_style.add_modifier(Modifier::CROSSED_OUT);
        }

        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}. {: <}", i + 1, item.name),
            item_style
        ))));
    }

    let list = List::new(list_items);

    f.render_widget(list, chunks[1]);

    let current_key_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "\"q\" to quit | \"d\" to delete | \"Enter\" to select/toggle completion | \"Up\"/\"Down\" move cursor",
                Style::default().fg(Color::Red)
            ),
            CurrentScreen::Adding => Span::styled(
                "\"ESC\" to exit | \"Enter\" to submit",
                Style::default().fg(Color::Red)
            ),
        }
    };

    let key_notes_footer = Paragraph::new(Line::from(current_key_hint))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(key_notes_footer, chunks[2]);
}

fn ui_adding(f: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .title("Enter new todo entry")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 25, f.size());

    f.render_widget(popup_block, area);

    let popup_chunk = Layout::new(Direction::Horizontal, [
        Constraint::Percentage(100)
    ])
        .margin(1)
        .split(area);

    let entry_block = Block::default()
        .title("Todo Entry")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::LightYellow).fg(Color::Black));

    let entry_text = Paragraph::new(app.input.clone()).block(entry_block);
    f.render_widget(entry_text, popup_chunk[0])
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::new(Direction::Vertical, [
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ]).split(r);

    // Then cut the middle vertical piece into three width-size pieces
    Layout::new(Direction::Horizontal, [
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ]).split(popup_layout[1])[1]// Return the middle chunk
}