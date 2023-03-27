use core::mem::swap;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame, Terminal,
};
use rand::Rng;

struct App {
    x_val: Vec<f64>,
    val: Vec<f64>,
    data: Vec<(f64, f64)>,
}

impl App {
    fn new() -> App {
        App {
            x_val: vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0],
            val: vec![1.0,2.0,3.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0],
            data: vec![(0.0,0.0),(0.0,0.0),(0.0,0.0)],
        }
    }

    fn on_tick(&mut self) {
        self.val.remove(0);
        let mut rng = rand::thread_rng();
        let v: f64 = rng.gen_range(0.0..10.0);
        self.val.push(v);
        let mut xbuf = Vec::new();
        swap(&mut xbuf, &mut self.x_val);
        let mut vbuf = Vec::new();
        swap(&mut vbuf, &mut self.val);
        let d: Vec<(f64, f64)> = xbuf.into_iter().zip(vbuf.into_iter()).collect();
        self.data = d;
   }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .direction(Direction::Horizontal)
        .split(f.size());

    let datasets = vec![Dataset::default()
        .name("data")
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Yellow))
        .graph_type(GraphType::Line)
        .data(&app.data)];
    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title(Span::styled(
                    "Chart 2",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("2.5"),
                    Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 5.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("2.5"),
                    Span::styled("5.0", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    f.render_widget(chart, chunks[0]);
}

