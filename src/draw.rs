use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{Frame, Terminal};

use crate::app::{App, Widgets};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	terminal
		.draw(|mut frame| {
			if let Some(statusbar) = app.statusbar.as_mut() {
				let chunks = Layout::default()
					.constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
					.split(frame.size());
				draw_widgets(&mut frame, &mut app.widgets, chunks[0]);
				frame.render_widget(statusbar, chunks[1]);
			} else {
				let chunks = Layout::default()
					.constraints(vec![Constraint::Percentage(100)])
					.split(frame.size());
				draw_widgets(&mut frame, &mut app.widgets, chunks[0]);
			}
		})
		.unwrap();
}

pub fn draw_widgets<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3), 
                Constraint::Ratio(1, 3), 
                Constraint::Ratio(1, 3) 
            ].as_ref()
        )
        .split(area);
        
    let mut top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(chunks[0]);

    if let Some(battery) = widgets.battery.as_ref() {
        frame.render_widget(battery, top_row[1]);
    } else {
        top_row = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(chunks[0]);
    }

    frame.render_widget(&widgets.cpu, top_row[0]);
    
    let middle_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref())
        .split(chunks[1]);
    frame.render_widget(&widgets.mem, middle_row[1]);        
    let mid_vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(middle_row[0]);
    frame.render_widget(widgets.disk.as_ref().unwrap(), mid_vert[0]);
    frame.render_widget(widgets.temp.as_ref().unwrap(), mid_vert[1]);
        
    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(chunks[2]);
    frame.render_widget(widgets.net.as_ref().unwrap(), bottom_row[0]);
    frame.render_widget(&mut widgets.proc, bottom_row[1]);           
}

pub fn draw_help_menu<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	terminal
		.draw(|mut frame| {
			let rect = app.help_menu.get_rect(frame.size());
			frame.render_widget(&app.help_menu, rect);
		})
		.unwrap();
}

// TODO: figure out how to draw the proc widget without clearing rest of the screen
pub fn draw_proc<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	draw(terminal, app);
	// terminal.draw(|mut frame| {
	// 	let chunks = if app.statusbar.is_some() {
	// 		Layout::default()
	// 			.constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
	// 			.split(frame.size())
	// 	} else {
	// 		Layout::default()
	// 			.constraints(vec![Constraint::Percentage(100)])
	// 			.split(frame.size())
	// 	};

	// 	let vertical_chunks = if app.widgets.temp.is_some() {
	// 		Layout::default()
	// 			.direction(Direction::Vertical)
	// 			.constraints(
	// 				[
	// 					Constraint::Ratio(1, 3),
	// 					Constraint::Ratio(1, 3),
	// 					Constraint::Ratio(1, 3),
	// 				]
	// 				.as_ref(),
	// 			)
	// 			.split(chunks[0])
	// 	} else {
	// 		Layout::default()
	// 			.direction(Direction::Vertical)
	// 			.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
	// 			.split(chunks[0])
	// 	};

	// 	let horizontal_chunks = Layout::default()
	// 		.direction(Direction::Horizontal)
	// 		.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
	// 		.split(*vertical_chunks.last().unwrap());
	// 	app.widgets.proc.render(&mut frame, horizontal_chunks[1]);
	// })
}

// TODO: figure out how to draw the graphs without clearing rest of the screen
pub fn draw_graphs<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	draw(terminal, app);
}
