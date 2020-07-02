// use crate::cli::App;

// use tui::{
//     backend::Backend,
//     layout::{Constraint, Direction, Layout, Rect},
//     style::{Color, Modifier, Style},
//     symbols,
//     widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
//     widgets::{
//         Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, Paragraph, Row, Sparkline,
//         Table, Tabs, Text,
//     },
//     Frame,
// };

// pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
//     let chunks = Layout::default()
//         .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
//         .split(f.size());
//     draw_main_tab(f, app, chunks[1]);
// }

// fn draw_main_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
// where
//     B: Backend,
// {
//     let chunks = Layout::default()
//         .constraints(
//             [
//                 Constraint::Length(7),
//                 Constraint::Min(7),
//                 Constraint::Length(7),
//             ]
//             .as_ref(),
//         )
//         .split(area);
//     draw_text(f, chunks[2]);
// }

// fn draw_text<B>(f: &mut Frame<B>, area: Rect)
// where
//     B: Backend,
// {
//     let text = [
//         Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFor example: "),
//         Text::styled("under", Style::default().fg(Color::Red)),
//         Text::raw(" "),
//         Text::styled("the", Style::default().fg(Color::Green)),
//         Text::raw(" "),
//         Text::styled("rainbow", Style::default().fg(Color::Blue)),
//         Text::raw(".\nOh and if you didn't "),
//         Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
//         Text::raw(" you can "),
//         Text::styled("automatically", Style::default().modifier(Modifier::BOLD)),
//         Text::raw(" "),
//         Text::styled("wrap", Style::default().modifier(Modifier::REVERSED)),
//         Text::raw(" your "),
//         Text::styled("text", Style::default().modifier(Modifier::UNDERLINED)),
//         Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
//     ];
//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title("Footer")
//         .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD));
//     let paragraph = Paragraph::new(text.iter()).block(block).wrap(true);
//     f.render_widget(paragraph, area);
// }
