pub mod event;
pub mod key;
pub mod handlers;
pub mod util;

pub use key::Key;

use crate::app::{ActiveBlock, App, MAJOR_INDICES, OrderFormState, RouteId};
use util::{get_color, date_from_timestamp};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Row, Table, Wrap},
    Frame,
};

// pub enum TableId {
//     TickerDetail,
//     TickerList,
//     RecentlySearched,
// }

// #[derive(PartialEq)]
// pub enum ColumnId {
//     None,
//     Symbol,
//     SecurityType,
// }

// impl Default for ColumnId {
//     fn default() -> Self {
//         ColumnId::None
//     }
// }

// pub struct TableHeader<'a> {
//     id: TableId,
//     items: Vec<TableHeaderItem<'a>>,
// }

// impl TableHeader<'_> {
//     pub fn get_index(&self, id: ColumnId) -> Option<usize> {
//         self.items.iter().position(|item| item.id == id)
//     }
// }

// #[derive(Default)]
// pub struct TableHeaderItem<'a> {
//     id: ColumnId,
//     text: &'a str,
//     width: u16,
// }

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

    draw_sidebar_block(f, app, chunks[0]);
    draw_a_route(f, app, chunks[1]);
}

pub fn draw_a_route<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::Error => {
            draw_error(f, app, layout_chunk)
        }
        RouteId::OrderForm => {
            draw_order_form(f, app, layout_chunk)
        }
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

pub fn draw_error<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(layout_chunk);
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );

    let welcome = Block::default()
        .title(Span::styled("Something when wrong", get_color(highlight_state, app.user_config.theme)))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.user_config.theme));
    f.render_widget(welcome, layout_chunk);

    let top_text = Paragraph::new("Esc to go back.")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default());
    f.render_widget(top_text, chunks[0]);
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

    let rows = [i0, i1, i2, i3, i4, i5]
        .iter()
        .map(|i| Row::new(i.data.clone()).style(style).height(3))
        .collect::<Vec<Row>>();

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

pub fn draw_order_form<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::OrderForm,
        current_route.hovered_block == ActiveBlock::OrderForm,
        );

    let mut text = vec![];
    if let Some(ref order_form) = app.preview_order_form {
        let order_action = order_form.order_action.to_string();
        text.push(
            Spans::from(vec![
                        Span::raw("Order Action ➤ "),
                        Span::styled(order_action, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ])
        );

        let symbol = app.preview_order_ticker.as_ref().unwrap_or(&"Error".to_string()).to_string();
        text.push(
            Spans::from(vec![
                        Span::raw("Symbol ➤ "),
                        Span::styled(symbol, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ])
        );
        text.push(
            Spans::from(vec![
                        Span::raw(" "),
                        Span::raw("Order Type ➤ "),
                        Span::styled(order_form.order_type.to_string(), Style::default().add_modifier(Modifier::BOLD))
            ])
        );
    }

    if let OrderFormState::Quantity = app.order_form_state {
        text.push(
            Spans::from(vec![
                        Span::raw("1. Input number of shares"),
            ]),

            );
    } else if let OrderFormState::Submit = app.order_form_state {
        if let Some(ref order_form) = app.preview_order_form {
            text.push(
                Spans::from(vec![
                            Span::raw("1. Number of shares"),
                            Span::raw(": "),
                            Span::raw(order_form.quantity.to_owned()),
                ])
                );
            text.push(
                Spans::from(vec![
                            Span::raw("2. Yay! Press Enter to Submit"),
                ])
                );
        }
    }

    let input = Paragraph::new(text).block(
        Block::default()
        .title("Order Form")
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state, app.user_config.theme)),
    ).wrap(Wrap { trim: true });

    f.render_widget(input, layout_chunk);
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

    let search_results = &app.search_results;
    if search_results.tickers.is_none() {
        return;
    }

    let list_items: Vec<ListItem> = search_results.tickers
        .iter()
        .flatten()
        .map(|i| ListItem::new(Span::raw(i.symbol.to_string())))
        .collect();

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
}

pub fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
    where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(98)].as_ref())
        .margin(2)
        .split(layout_chunk);

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

    // Banner text with correct styling
    let mut top_text = Text::from("Accounts");
    top_text.patch_style(Style::default().fg(Color::Yellow));

    // Contains the banner
    let top_text = Paragraph::new(top_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default());
    f.render_widget(top_text, chunks[0]);


    if let Some(ref user_accounts) = app.user_accounts {
        let mut bottom_text = String::new();
        for acc in user_accounts {
            bottom_text.push_str(" ➤ ");
            bottom_text.push_str(&acc.account_id);
            bottom_text.push_str("\n    ");
            bottom_text.push_str(&acc.account_id_key);
            bottom_text.push_str("\n    ");
            if !acc.account_name.is_empty() {
                bottom_text.push_str(&acc.account_name);
                bottom_text.push_str("\n    ");
            }
            if let Some(balance) = &acc.account_balance {
                if let Some(computed) = &balance.computed {
                    bottom_text.push_str("Account Value: ");
                    bottom_text.push_str(&computed.real_time_values.total_account_value);
                    bottom_text.push_str("\n    ");
                    bottom_text.push_str("Net Market Value: ");
                    bottom_text.push_str(&computed.real_time_values.net_mv);
                    bottom_text.push_str("\n    ");
                }
            }
            bottom_text.push_str(&acc.account_type);

            // TODO: add account balance
            //
            bottom_text.push_str("\n\n");
        }
        let bottom_text = Paragraph::new(bottom_text)
            .style(Style::default().fg(app.user_config.theme.text))
            .block(Block::default())
            .wrap(Wrap { trim: false });
        f.render_widget(bottom_text, chunks[1]);
    }
}

fn draw_sidebar_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
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
    state.select(app.selected_watch_list_index);

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
    let title = match current_route.id {
        RouteId::OrderForm => {
            match app.order_form_state {
                OrderFormState::Quantity => {
                    "No. of shares"
                }
                _ => {
                    "Preview Order"
                }
            }
        }
        _ => "Search"
    };
    let lines = Text::from((&input_string).as_str());
    let input = Paragraph::new(lines).block(
        Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(title, get_color(highlight_state, app.user_config.theme)))
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
