pub mod event;
pub mod key;

use crate::app::{App, BANNER, MAJOR_INDICES};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw_main<B>(f: &mut Frame<B>, app: App)
    where B: Backend,
          {
              let parent_layout = Layout::default()
                  .direction(Direction::Vertical)
                  .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                  .margin(1)
                  .split(f.size());

              draw_input_and_help_box(f, &app, parent_layout[0]);
              // Nested main block with potential routes
              draw_user_blocks(f, &app, parent_layout[1]);
          }

pub fn draw_user_blocks<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_portfolio_block(f, app, chunks[0]);
    draw_splash(f, app, chunks[1]);
}

pub fn draw_splash<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let style = Style::default().fg(Color::Yellow);
    let welcome = Block::default()
        .title(Span::styled(
            "Welcome!",
            style,
        ))
        .borders(Borders::ALL)
        .border_style(style);
    f.render_widget(welcome, layout_chunk);

    // Banner text with correct styling
    let mut top_text = Text::from(BANNER);
    top_text.patch_style(Style::default().fg(Color::Yellow));

    let bottom_text_raw = format!(
        "{}",
        "\nPlease report any bugs or missing features to https://github.com/snewcomer/stonks-terminal\n\n",
        );
    let bottom_text = Text::from(bottom_text_raw.as_str());

    // Contains the banner
    let top_text = Paragraph::new(top_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default());
    f.render_widget(top_text, chunks[0]);
}

pub fn draw_portfolio_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let mut state = ListState::default();
    state.select(Some(0));

    let lst_items: Vec<ListItem> = MAJOR_INDICES
        .iter()
        .map(|i| ListItem::new(Span::raw(*i)))
        .collect();

    let style = Style::default().fg(Color::Yellow);
    let list = List::new(lst_items)
        .block(
            Block::default()
            .title(Span::styled("Major Averages", style))
            .borders(Borders::ALL)
            .border_style(style),
            )
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(style.add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, layout_chunk, &mut state);
}

pub fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    // Check for the width and change the contraints accordingly
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
        .split(layout_chunk);

    let input_string: String = app.input.iter().collect();
    let lines = Text::from((&input_string).as_str());
    let style = Style::default().fg(Color::Yellow);
    let input = Paragraph::new(lines).block(
        Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Search", style))
        .border_style(style),
        );
    f.render_widget(input, chunks[0]);

    let block = Block::default()
        .title(Span::styled("Help", Style::default().fg(Color::Yellow)))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let lines = Text::from("Type ?");
    let help = Paragraph::new(lines)
        .block(block)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(help, chunks[1]);
}

