use std::{collections::BTreeMap, rc::Rc};

use ratatui::{
  layout::{Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  text::{Line, Span},
  widgets::{Block, Borders},
};

// Utils

// default colors
pub const COLOR_TEAL: Color = Color::Rgb(35, 50, 55);
pub const COLOR_CYAN: Color = Color::Rgb(0, 230, 230);
pub const COLOR_LIGHT_BLUE: Color = Color::Rgb(138, 196, 255);
pub const COLOR_YELLOW: Color = Color::Rgb(249, 229, 113);
pub const COLOR_GREEN: Color = Color::Rgb(72, 213, 150);
pub const COLOR_RED: Color = Color::Rgb(249, 167, 164);
pub const COLOR_ORANGE: Color = Color::Rgb(255, 170, 66);
pub const COLOR_WHITE: Color = Color::Rgb(255, 255, 255);
// light theme colors
pub const COLOR_MAGENTA: Color = Color::Rgb(139, 0, 139);
pub const COLOR_GRAY: Color = Color::Rgb(91, 87, 87);
pub const COLOR_BLUE: Color = Color::Rgb(0, 82, 163);
pub const COLOR_GREEN_DARK: Color = Color::Rgb(20, 97, 73);
pub const COLOR_RED_DARK: Color = Color::Rgb(173, 25, 20);
pub const COLOR_ORANGE_DARK: Color = Color::Rgb(184, 49, 15);

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Styles {
  Default,
  Logo,
  Failure,
  Warning,
  Success,
  Primary,
  Secondary,
  Help,
  Background,
}

pub fn theme_styles(light: bool) -> BTreeMap<Styles, Style> {
  if light {
    BTreeMap::from([
      (Styles::Default, Style::default().fg(COLOR_GRAY)),
      (Styles::Logo, Style::default().fg(COLOR_GREEN_DARK)),
      (Styles::Failure, Style::default().fg(COLOR_RED_DARK)),
      (Styles::Warning, Style::default().fg(COLOR_ORANGE_DARK)),
      (Styles::Success, Style::default().fg(COLOR_GREEN_DARK)),
      (Styles::Primary, Style::default().fg(COLOR_BLUE)),
      (Styles::Secondary, Style::default().fg(COLOR_MAGENTA)),
      (Styles::Help, Style::default().fg(COLOR_BLUE)),
      (
        Styles::Background,
        Style::default().bg(COLOR_WHITE).fg(COLOR_GRAY),
      ),
    ])
  } else {
    BTreeMap::from([
      (Styles::Default, Style::default().fg(COLOR_WHITE)),
      (Styles::Logo, Style::default().fg(COLOR_GREEN)),
      (Styles::Failure, Style::default().fg(COLOR_RED)),
      (Styles::Warning, Style::default().fg(COLOR_ORANGE)),
      (Styles::Success, Style::default().fg(COLOR_GREEN)),
      (Styles::Primary, Style::default().fg(COLOR_CYAN)),
      (Styles::Secondary, Style::default().fg(COLOR_YELLOW)),
      (Styles::Help, Style::default().fg(COLOR_LIGHT_BLUE)),
      (
        Styles::Background,
        Style::default().bg(COLOR_TEAL).fg(COLOR_WHITE),
      ),
    ])
  }
}

pub fn title_style(txt: &str) -> Line<'_> {
  Line::from(vec![Span::styled(txt, style_bold())])
}

pub fn title_style_logo(txt: &str, light: bool) -> Span<'_> {
  Span::styled(
    txt,
    style_logo(light)
      .add_modifier(Modifier::BOLD)
      .add_modifier(Modifier::ITALIC),
  )
}

pub fn style_bold() -> Style {
  Style::default().add_modifier(Modifier::BOLD)
}

pub fn style_default(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Default).unwrap()
}
pub fn style_logo(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Logo).unwrap()
}
pub fn style_failure(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Failure).unwrap()
}
pub fn style_warning(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Warning).unwrap()
}
pub fn style_success(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Success).unwrap()
}
pub fn style_primary(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Primary).unwrap()
}
pub fn style_help(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Help).unwrap()
}

pub fn style_secondary(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Secondary).unwrap()
}

pub fn style_main_background(light: bool) -> Style {
  *theme_styles(light).get(&Styles::Background).unwrap()
}

pub fn style_highlight() -> Style {
  Style::default().add_modifier(Modifier::REVERSED)
}

pub fn horizontal_chunks(constraints: Vec<Constraint>, size: Rect) -> Rc<[Rect]> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Horizontal)
    .split(size)
}

pub fn horizontal_chunks_with_margin(
  constraints: Vec<Constraint>,
  size: Rect,
  margin: u16,
) -> Rc<[Rect]> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Horizontal)
    .margin(margin)
    .split(size)
}

pub fn vertical_chunks(constraints: Vec<Constraint>, size: Rect) -> Rc<[Rect]> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Vertical)
    .split(size)
}

pub fn vertical_chunks_with_margin(
  constraints: Vec<Constraint>,
  size: Rect,
  margin: u16,
) -> Rc<[Rect]> {
  Layout::default()
    .constraints(<Vec<Constraint> as AsRef<[Constraint]>>::as_ref(
      &constraints,
    ))
    .direction(Direction::Vertical)
    .margin(margin)
    .split(size)
}

pub fn layout_block(title: Span<'_>) -> Block<'_> {
  Block::default().borders(Borders::ALL).title(title)
}

pub fn layout_block_with_str(title: &str, light: bool, is_active: bool) -> Block<'_> {
  layout_block_with_line(title_style(title), light, is_active)
}

pub fn layout_block_with_line(title: Line<'_>, light: bool, is_active: bool) -> Block<'_> {
  let style = if is_active {
    style_secondary(light)
  } else {
    style_default(light)
  };

  Block::default()
    .borders(Borders::ALL)
    .title(title)
    .style(style)
}

pub fn title_with_dual_style<'a>(part_1: String, part_2: String) -> Line<'a> {
  Line::from(vec![
    Span::styled(part_1, Style::default().add_modifier(Modifier::BOLD)),
    Span::styled(part_2, Style::default()),
  ])
}
