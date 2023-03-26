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
    text::Span,
    widgets::{BarChart,Block, Borders, Gauge},
    Frame, Terminal,
};
struct App<'a> {
    progress1: u16,
    progress2: u16,
    progress3: f64,
    progress4: u16,
    bardata: Vec<(&'a str, u64)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            progress1: 0,
            progress2: 0,
            progress3: 0.45,
            progress4: 0,
            bardata: vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
                ("B10", 15),
                ("B11", 1),
                ("B12", 0),
                ("B13", 4),
                ("B14", 6),
                ("B15", 4),
                ("B16", 6),
                ("B17", 4),
                ("B18", 7),
                ("B19", 13),
                ("B20", 8),
                ("B21", 11),
                ("B22", 9),
                ("B23", 3),
                ("B24", 5),
            ],
        }
    }

    fn on_tick(&mut self) {
        self.progress1 += 1;
        if self.progress1 > 100 {
            self.progress1 = 0;
        }
        self.progress2 += 2;
        if self.progress2 > 100 {
            self.progress2 = 0;
        }
        self.progress3 += 0.001;
        if self.progress3 > 1.0 {
            self.progress3 = 0.0;
        }
        self.progress4 += 1;
        if self.progress4 > 100 {
            self.progress4 = 0;
        }
        let value = self.bardata.pop().unwrap();
        self.bardata.insert(0, value);
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
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(f.size());

    let gaugechunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let barchunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let gauge = Gauge::default()
        .block(Block::default().title("Gauge1").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(app.progress1);

    f.render_widget(gauge, gaugechunks[0]);

    let label = format!("{}/100", app.progress2);
    let gauge = Gauge::default()
        .block(Block::default().title("Gauge2").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Green))
        .percent(app.progress2)
        .label(label);

    f.render_widget(gauge, gaugechunks[1]);

    let barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .data(&app.bardata)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, barchunks[0]);

    let barchart = BarChart::default()
        .block(Block::default().title("Data2").borders(Borders::ALL))
        .data(&app.bardata)
        .bar_width(5)
        .bar_gap(3)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(
            Style::default()
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(barchart, barchunks[1]);

    
}
