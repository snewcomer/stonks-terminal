pub mod event;
pub mod key;
pub mod handlers;
pub mod util;

pub use key::Key;

use crate::app::{ActiveBlock, App, MAJOR_INDICES, RouteId};
use util::{get_color, get_percentage_width, date_from_timestamp};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
    Frame,
};

pub enum TableId {
  TickerDetail,
  TickerList,
  RecentlySearched,
}

#[derive(PartialEq)]
pub enum ColumnId {
  None,
  Symbol,
  SecurityType,
}

impl Default for ColumnId {
  fn default() -> Self {
    ColumnId::None
  }
}

pub struct TableHeader<'a> {
  id: TableId,
  items: Vec<TableHeaderItem<'a>>,
}

impl TableHeader<'_> {
  pub fn get_index(&self, id: ColumnId) -> Option<usize> {
    self.items.iter().position(|item| item.id == id)
  }
}

#[derive(Default)]
pub struct TableHeaderItem<'a> {
  id: ColumnId,
  text: &'a str,
  width: u16,
}

pub struct TableItem {
  id: String,
  data: Vec<String>,
}

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
    draw_a_route(f, app, chunks[1]);
}

pub fn draw_a_route<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::TickerDetail if app.selected_ticker.is_some() => {
           draw_ticker_detail(f, app, layout_chunk)
        }
        RouteId::Search => {
           draw_search_results(f, app, layout_chunk)
        }
        RouteId::Home => {
           draw_home(f, app, layout_chunk)
        }
        _ => draw_home(f, app, layout_chunk)

    }
}

pub fn draw_ticker_detail<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::TickerDetail,
        current_route.hovered_block == ActiveBlock::TickerDetail,
    );

    let selected_ticker = app.selected_ticker.as_ref();
    let ticker = &selected_ticker.unwrap().ticker;

    // let header = TableHeader {
    //     id: TableId::TickerDetail,
    //     items: vec![
    //         TableHeaderItem {
    //             id: ColumnId::PrimaryExchange,
    //             text: "Exchange",
    //             // We need to subtract the fixed value of the previous column
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 2,
    //         },
    //         TableHeaderItem {
    //             id: ColumnId::Bid,
    //             text: "Bid",
    //             // We need to subtract the fixed value of the previous column
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 5.0) - 2,
    //         },
    //         TableHeaderItem {
    //             id: ColumnId::Ask,
    //             text: "Ask",
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
    //             ..Default::default()
    //         },
    //         TableHeaderItem {
    //             id: ColumnId::PE,
    //             text: "PE",
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
    //         },
    //         TableHeaderItem {
    //             id: ColumnId::EPS,
    //             text: "EPS",
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
    //         },
    //         TableHeaderItem {
    //             id: ColumnId::Dividend,
    //             text: "dividend",
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 5.0),
    //         },
    //     ],
    // };

    let style = Style::default().fg(app.user_config.theme.text); // default styling

    let i0 = TableItem {
        id: ticker.symbol.to_owned(),
        data: vec![
            format!("{} ➤ {}", "exch", ticker.primary_exchange.to_owned()),
            format!("{} ➤ {}", "date", ticker.date_time.to_owned()),
        ]
    };

    let i1 = TableItem {
        id: ticker.symbol.to_owned(),
        data: vec![
            format!("{}  |  ${}", "bid", ticker.bid.to_owned()),
            format!("{}  |  ${}", "ask", ticker.ask.to_owned()),
            format!("{}  |  ${}", "open", ticker.high52.to_owned()),
        ]
    };

    let i2 = TableItem {
        id: ticker.symbol.to_owned(),
        data: vec![
            format!("{}  |  {}", "eps", ticker.eps.to_owned()),
            format!("{}  |  {}", "pe", ticker.pe.to_owned()),
            format!("{}  |  {}", "beta", ticker.beta.to_owned()),
        ]
    };
    let i3 = TableItem {
        id: ticker.symbol.to_owned(),
        data: vec![
            format!("{}  |  ${}", "high 52", ticker.high52.to_owned()),
            format!("on {}", date_from_timestamp(ticker.week52_hi_date)),
        ]
    };

    let i4 = TableItem {
        id: ticker.symbol.to_owned(),
        data: vec![
            format!("{}  |  ${}", "low 52", ticker.low52.to_owned()),
            format!("on {}", date_from_timestamp(ticker.week52_low_date)),
        ]
    };

    let i5 = TableItem {
        id: ticker.symbol.to_owned(),
        data: vec![
            format!("{}  |  ${}", "dividend", ticker.dividend.to_owned()),
            format!("{} ➤ {}", "ex dividend date", date_from_timestamp(ticker.ex_dividend_date)),
        ]
    };

    let rows = [i0, i1, i2, i3, i4, i5].iter().map(|i| Row::new(i.data.clone()).style(style).height(3)).collect::<Vec<Row>>();

    // let widths = header
    //     .items
    //     .iter()
    //     .map(|h| Constraint::Length(h.width))
    //     .collect::<Vec<tui::layout::Constraint>>();

    let table = Table::new(rows)
        // .header(
        //   Row::new(header.items.iter().map(|h| h.text))
        //     .style(Style::default().fg(app.user_config.theme.header)),
        // )
        .block(
          Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(app.user_config.theme.text))
            .title(Span::styled(
              ticker.symbol.to_owned(),
              get_color(highlight_state, app.user_config.theme),
            ))
            .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        // .widths(&widths);
        .widths(&[Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)]);

    f.render_widget(table, layout_chunk);
}

pub fn draw_search_results<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let mut state = ListState::default();
    state.select(app.search_results.selected_ticker_index);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::SearchResults,
        current_route.hovered_block == ActiveBlock::SearchResults,
    );

    // let header = TableHeader {
    //     id: TableId::TickerDetail,
    //     items: vec![
    //         TableHeaderItem {
    //             id: ColumnId::Symbol,
    //             text: "Symbol",
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 4.0),
    //         },
    //         TableHeaderItem {
    //             id: ColumnId::SecurityType,
    //             text: "Security Type",
    //             // We need to subtract the fixed value of the previous column
    //             width: get_percentage_width(layout_chunk.width, 2.0 / 4.0) - 2,
    //         },
    //     ],
    // };

    // let style = Style::default().fg(app.user_config.theme.text); // default styling

    let search_results = &app.search_results;
    if search_results.tickers.is_none() {
        return;
    }

    let list_items: Vec<ListItem> = search_results.tickers
        .iter()
        .flatten()
        .map(|i| ListItem::new(Span::raw(i.symbol.to_string())))
        .collect();

    // let items = search_results.tickers.as_ref().unwrap().into_iter().map(|res| {
    //     TableItem {
    //         id: res.symbol.to_owned(),
    //         data: vec![
    //             format!("{}  |  {}", "symbol", res.symbol.to_owned()),
    //             format!("{}  |  {}", "security type", res.security_type.to_owned()),
    //         ]
    //     }
    // });

    // let rows = items
    //     .map(|i| Row::new(i.data.clone()).style(style).height(3))
    //     .collect::<Vec<Row>>();

    // let widths = header
    //     .items
    //     .iter()
    //     .map(|h| Constraint::Length(h.width))
    //     .collect::<Vec<tui::layout::Constraint>>();

    let list = List::new(list_items)
        .block(
            Block::default()
            .title(Span::styled("Search Results", get_color(highlight_state, app.user_config.theme)))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .style(Style::default().fg(app.user_config.theme.text))
        .highlight_style(get_color(highlight_state, app.user_config.theme).add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, layout_chunk, &mut state);
    // let table = Table::new(rows)
    //     .header(
    //       Row::new(header.items.iter().map(|h| h.text))
    //         .style(Style::default().fg(app.user_config.theme.header)),
    //     )
    //     .block(
    //       Block::default()
    //         .borders(Borders::ALL)
    //         .style(Style::default().fg(app.user_config.theme.text))
    //         .title(Span::styled(
    //           app.search_term.to_owned(),
    //           get_color(highlight_state, app.user_config.theme),
    //         ))
    //         .border_style(get_color(highlight_state, app.user_config.theme)),
    //     )
    //     .style(Style::default().fg(app.user_config.theme.text))
    //     .widths(&widths);

    // f.render_stateful_widget(table, layout_chunk, &mut state);
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

    let list_items: Vec<ListItem> = MAJOR_INDICES
        .iter()
        .map(|i| ListItem::new(Span::raw(*i)))
        .collect();

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::WatchList,
        current_route.hovered_block == ActiveBlock::WatchList,
    );

    let list = List::new(list_items)
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

    if let Some(tickers) = &app.portfolio_tickers {
        let list_items: Vec<ListItem> = tickers
            .iter()
            .map(|i| ListItem::new(Span::raw(i.symbol.to_string())))
            .collect();

        let current_route = app.get_current_route();
        let highlight_state = (
            current_route.active_block == ActiveBlock::Portfolio,
            current_route.hovered_block == ActiveBlock::Portfolio,
        );

        let list = List::new(list_items)
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
