use std::error::Error;
use std::io::Write;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{self as ct, event::Event as CEvent, event::KeyCode, event::KeyEvent, execute};

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::canvas::{Canvas, Points};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::{self, Frame};

use crate::naive::{Cell, Game};
use crate::Rules;

pub fn run() -> Result<(), Box<dyn Error>> {
    // TODO: This setup (aka. input handler sending ticks) is probably bad for
    // GOL, since the game itself should be able to handle 100fps (10ms ticks),
    // while input can be much slower, but maybe it works?
    let mut term = Term::new()?;
    let input_receiver = setup_input_handler();
    let mut game = setup_game();
    loop {
        term.draw(&game)?;
        let stop_running = handle_input(&input_receiver, &mut game)?;
        if stop_running {
            break;
        }
    }
    Ok(())
}

enum Event<I> {
    Input(I),
    Tick,
}

struct Term {
    // TODO: Maybe implement Deref trait, so it can be used as tui::Terminal?
    terminal: tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>,
}

impl Term {
    fn new() -> Result<Term, Box<dyn Error>> {
        // TODO: Return better error

        // Build terminal
        let stdout = std::io::stdout();
        let backend = tui::backend::CrosstermBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend)?;

        // Configure terminal
        ct::terminal::enable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        );

        Ok(Term { terminal })
    }

    fn draw(&mut self, game: &Game) -> Result<(), Box<dyn Error>> {
        self.terminal.draw(|frame| Term::draw_ui(frame, &game))?;
        Ok(())
    }

    fn draw_ui(frame: &mut Frame<impl tui::backend::Backend>, game: &Game) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3), // Header/Menu
                    Constraint::Min(2),    // Game window
                    Constraint::Length(3), // Footer
                ]
                .as_ref(),
            )
            .split(size);
        let header = Paragraph::new("GoL - Game of Life")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Header Block")
                    .border_type(BorderType::Plain),
            );
        //let inner_area = canvas_block.inner(chunks[1]);
        //let (width, height) = (inner_area.width as f64, inner_area.height as f64);
        let live_cells = game.get_alive_cells();
        let live_cells = live_cells
            .iter()
            .map(|(x, y)| (*x as f64, *y as f64))
            .collect::<Vec<(f64, f64)>>();
        let game_world = Canvas::default()
            //.marker(symbols::Marker::Dot)
            .block(Block::default().title("Game World").borders(Borders::ALL))
            .x_bounds([0.0, game.get_width() as f64])
            .y_bounds([0.0, game.get_height() as f64])
            .paint(|context| {
                context.draw(&Points {
                    coords: &live_cells,
                    color: Color::White,
                })
            });
        let footer = Paragraph::new(
            "↑←↓→|WASD: Move game window - Space: Start/Stop Game - Mouse: Toggle cell status",
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Controls")
                .border_type(BorderType::Plain),
        );
        frame.render_widget(header, chunks[0]);
        frame.render_widget(game_world, chunks[1]);
        frame.render_widget(footer, chunks[2]);
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        execute!(
            self.terminal.backend_mut(),
            DisableMouseCapture,
            LeaveAlternateScreen
        );
        ct::terminal::disable_raw_mode();
    }
}

fn setup_input_handler() -> Receiver<Event<KeyEvent>> {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(100); // TODO: make speed adjustable
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if ct::event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = ct::event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                let res = tx.send(Event::Tick);
                if res.is_ok() {
                    last_tick = Instant::now();
                }
            }
        }
    });
    rx
}

// TODO: Maybe move into naive renderer? Make interface/trait so different Game versions can be launched
fn setup_game() -> Game {
    let mut game = Game::new(10, 10, Rules::default());

    game[0][1] = Cell::Alive;
    game[1][2] = Cell::Alive;
    game[2][0] = Cell::Alive;
    game[2][1] = Cell::Alive;
    game[2][2] = Cell::Alive;

    game
}

fn handle_input(
    input_receiver: &Receiver<Event<KeyEvent>>,
    game: &mut Game,
) -> Result<bool, Box<dyn Error>> {
    // Blocks until Event is available
    match input_receiver.recv()? {
        Event::Input(event) => {
            let stop_running = handle_keypress(event.code);
            if stop_running {
                return Ok(true);
            }
        }
        Event::Tick => game.compute_next_gen(),
    }

    Ok(false)
}

fn handle_keypress(key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => {
            return true;
        }
        KeyCode::Char(' ') => {
            // Start Simulation
        }
        _ => {}
    }

    false
}
