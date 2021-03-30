pub mod event;
pub mod key;
pub mod handlers;
pub mod util;

pub use key::Key;

use crate::app::{ActiveBlock, App, MAJOR_INDICES};
use util::get_color;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn draw_main<B>(f: &mut Frame<B>, app: &App)
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

    draw_tickers_block(f, app, chunks[0]);
    draw_home(f, app, chunks[1]);
}

pub fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    // let chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .constraints([Constraint::Length(7), Constraint::Length(93)].as_ref())
    //     .margin(2)
    //     .split(layout_chunk);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );

    let welcome = Block::default()
        .title(Span::styled("Stats", get_color(highlight_state, app.user_config.theme)))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.user_config.theme));
    f.render_widget(welcome, layout_chunk);

    // // Banner text with correct styling
    // let mut top_text = Text::from(BANNER);
    // top_text.patch_style(Style::default().fg(Color::Yellow));

    // // Contains the banner
    // let top_text = Paragraph::new(top_text)
    //     .style(Style::default().fg(Color::Yellow))
    //     .block(Block::default());
    // f.render_widget(top_text, chunks[0]);
}

fn draw_tickers_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_watch_list_block(f, app, chunks[0]);
    draw_portfolio_block(f, app, chunks[1]);
}

pub fn draw_watch_list_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let mut state = ListState::default();
    state.select(Some(app.library.selected_index));

    let lst_items: Vec<ListItem> = MAJOR_INDICES
        .iter()
        .map(|i| ListItem::new(Span::raw(*i)))
        .collect();

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::WatchList,
        current_route.hovered_block == ActiveBlock::WatchList,
    );

    let list = List::new(lst_items)
        .block(
            Block::default()
            .title(Span::styled("Watch List", get_color(highlight_state, app.user_config.theme)))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .highlight_style(get_color(highlight_state, app.user_config.theme).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, layout_chunk, &mut state);
}

pub fn draw_portfolio_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let mut state = ListState::default();
    state.select(app.selected_ticker_index);

    let lst_items: Vec<ListItem> = MAJOR_INDICES
        .iter()
        .map(|i| ListItem::new(Span::raw(*i)))
        .collect();

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Portfolio,
        current_route.hovered_block == ActiveBlock::Portfolio,
    );

    let list = List::new(lst_items)
        .block(
            Block::default()
            .title(Span::styled("Portfolio", get_color(highlight_state, app.user_config.theme)))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .highlight_style(get_color(highlight_state, app.user_config.theme).add_modifier(Modifier::BOLD));
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

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Input,
        current_route.hovered_block == ActiveBlock::Input,
    );

    let input_string: String = app.input.iter().collect();
    let lines = Text::from((&input_string).as_str());
    let input = Paragraph::new(lines).block(
        Block::default()
        .borders(Borders::ALL)
        .title(Span::styled("Search", get_color(highlight_state, app.user_config.theme)))
        .border_style(get_color(highlight_state, app.user_config.theme)),
    );
    f.render_widget(input, chunks[0]);

    let block = Block::default()
        .title(Span::styled("Help", get_color(highlight_state, app.user_config.theme)))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.user_config.theme));

    let lines = Text::from("Type ?");
    let help = Paragraph::new(lines)
        .block(block)
        .style(get_color(highlight_state, app.user_config.theme));
    f.render_widget(help, chunks[1]);
}
