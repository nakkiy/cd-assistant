use std::io::Stderr;
use std::time::Instant;
use std::{env, fs, io, path::Path};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::popup::Popup;
use crate::tree::{DirEntry, LinkStatus};

pub struct App {
    root: DirEntry,
    focus_path: Vec<usize>,
    scroll_offset: usize,
    popup: Popup,
    search_buffer: String,
    last_input_time: Option<Instant>,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let current = env::current_dir()?;
        let mut ancestors: Vec<_> = vec![];
        let mut p = current.as_path();
        while p != Path::new("/") {
            ancestors.push(p.to_path_buf());
            if let Some(parent) = p.parent() {
                p = parent;
            } else {
                break;
            }
        }
        ancestors.push("/".into());
        ancestors.reverse();

        let mut root = DirEntry::new("/".into());
        root.expanded = true;

        let mut node = &mut root;
        let mut focus_path = vec![];
        for (depth, path) in ancestors.iter().skip(1).enumerate() {
            if depth == ancestors.len() - 2 {
                node.load_children();
                if let Some(i) = node
                    .children
                    .iter_mut()
                    .position(|c| path.starts_with(&c.path))
                {
                    node.children[i].expanded = true;
                    focus_path.push(i);
                    node = &mut node.children[i];
                }
            } else if let Some(i) = node.load_only(path) {
                node.children[i].expanded = true;
                focus_path.push(i);
                node = &mut node.children[i];
            }
        }

        {
            let mut node = &mut root;
            for &i in &focus_path {
                node = &mut node.children[i];
            }
            node.load_children();
        }

        Ok(Self {
            root,
            focus_path,
            scroll_offset: 0,
            popup: Popup::new(),
            search_buffer: String::new(),
            last_input_time: None,
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = self.init_terminal()?;

        loop {
            let mut lines = vec![];
            self.walk(&self.root, "".to_string(), &mut lines, vec![]);

            let height = terminal.size()?.height as usize;

            terminal.draw(|f| {
                let area = f.area();
                let items: Vec<ListItem> = lines
                    .iter()
                    .enumerate()
                    .skip(self.scroll_offset)
                    .take(area.height as usize)
                    .map(|(_i, (text, node, path))| {
                        let mut style = Style::default();
                        if self.focus_path == *path {
                            style = style.bg(Color::Rgb(40, 40, 40)).fg(Color::White);
                        }

                        style = match node.link_status {
                            LinkStatus::SymlinkOk => style.fg(Color::Cyan),
                            LinkStatus::SymlinkBroken => style.fg(Color::Red),
                            LinkStatus::Normal => style,
                        };

                        ListItem::new(text.clone()).style(style)
                    })
                    .collect();

                let list = List::new(items).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray)),
                );
                f.render_widget(list, area);

                if self.popup.visible {
                    let mut node = &self.root;
                    for &i in &self.focus_path {
                        node = &node.children[i];
                    }
                    self.popup.draw(f, area, node);
                }
            })?;

            if let Some(pos) = lines.iter().position(|(_, _, p)| *p == self.focus_path) {
                if pos < self.scroll_offset {
                    self.scroll_offset = pos;
                } else if pos >= self.scroll_offset + height.saturating_sub(3) {
                    self.scroll_offset = pos.saturating_sub(height.saturating_sub(3));
                }
            }

            if event::poll(std::time::Duration::from_millis(100))? {
                let evt = event::read()?;
                if let Some(output) = self.handle_event(evt, height)? {
                    self.cleanup_terminal()?;
                    if !output.is_empty() {
                        println!("{}", output);
                    }
                    return Ok(());
                }
            }
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn walk<'a>(
        &'a self,
        node: &'a DirEntry,
        prefix: String,
        lines: &mut Vec<(String, &'a DirEntry, Vec<usize>)>,
        path: Vec<usize>,
    ) {
        let marker = if node.expanded { "▼" } else { "▶" };
        let line = if node.path == Path::new("/") {
            format!("{} /", marker)
        } else if let Some(target) = &node.link_target {
            format!("{} {} -> {}", marker, node.name, target.display())
        } else {
            format!("{} {}", marker, node.name)
        };
        lines.push((format!("{}{}", prefix, line), node, path.clone()));
        if node.expanded {
            for (i, child) in node.children.iter().enumerate() {
                let mut new_path = path.clone();
                new_path.push(i);
                self.walk(child, format!("{}    ", prefix), lines, new_path);
            }
        }
    }

    fn move_focus(&mut self, direction: isize, height: usize) {
        let mut lines = vec![];
        self.walk(&self.root, "".to_string(), &mut lines, vec![]);

        let new_focus = lines
            .iter()
            .position(|(_, _, p)| *p == self.focus_path)
            .map(|pos| {
                let new = (pos as isize + direction).clamp(0, lines.len() as isize - 1) as usize;
                (lines[new].2.clone(), new)
            });

        if let Some((path, pos)) = new_focus {
            self.focus_path = path;
            if pos < self.scroll_offset {
                self.scroll_offset = pos;
            } else if pos >= self.scroll_offset + height.saturating_sub(3) {
                self.scroll_offset = pos.saturating_sub(height.saturating_sub(3));
            }
        }
    }

    fn handle_event(&mut self, event: Event, height: usize) -> io::Result<Option<String>> {
        use KeyCode::*;
        if self.popup.visible {
            let mut node = &self.root;
            for &i in &self.focus_path {
                node = &node.children[i];
            }

            if let Event::Key(key) = event {
                match key.code {
                    Char('q') | Esc | Char('f') => self.popup.hide(),
                    Char('j') | Down => self.popup.scroll_down(
                        fs::read_dir(&node.path).map(|r| r.count()).unwrap_or(0),
                        height / 2 - 2,
                    ),
                    Char('k') | Up => self.popup.scroll_up(),
                    _ => {}
                }
            }

            return Ok(None);
        }

        if let Event::Key(key) = event {
            match key.code {
                Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(Some("".to_string()));
                }
                Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.popup.toggle();
                }
                Enter => {
                    let mut node = &self.root;
                    for &i in &self.focus_path {
                        node = &node.children[i];
                    }
                    return Ok(Some(format!("cd {}", node.path.display())));
                }
                Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.move_focus(-1, height)
                }
                Up => self.move_focus(-1, height),
                Char('j') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.move_focus(1, height)
                }
                Down => self.move_focus(1, height),
                Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let mut node = &mut self.root;
                    for &i in &self.focus_path {
                        node = &mut node.children[i];
                    }
                    node.expanded = true;
                    node.load_children();
                }
                Right => {
                    let mut node = &mut self.root;
                    for &i in &self.focus_path {
                        node = &mut node.children[i];
                    }
                    node.expanded = true;
                    node.load_children();
                }
                Char('h') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let mut node = &mut self.root;
                    for &i in &self.focus_path {
                        node = &mut node.children[i];
                    }
                    if node.expanded {
                        node.collapse_all();
                    } else {
                        self.focus_path.pop();
                        let mut node = &mut self.root;
                        for &i in &self.focus_path {
                            node = &mut node.children[i];
                        }
                        node.load_children();
                    }
                }
                Left => {
                    let mut node = &mut self.root;
                    for &i in &self.focus_path {
                        node = &mut node.children[i];
                    }
                    if node.expanded {
                        node.collapse_all();
                    } else {
                        self.focus_path.pop();
                        let mut node = &mut self.root;
                        for &i in &self.focus_path {
                            node = &mut node.children[i];
                        }
                        node.load_children();
                    }
                }
                Char(c) if c.is_ascii_graphic() => {
                    self.handle_char_jump(c);
                }
                _ => {}
            }
        }

        Ok(None)
    }

    fn handle_char_jump(&mut self, c: char) {
        use std::time::{Duration, Instant};

        let now = Instant::now();
        let timeout = Duration::from_millis(1000);

        if let Some(last) = self.last_input_time {
            if now.duration_since(last) > timeout {
                self.search_buffer.clear();
            }
        }

        self.search_buffer.push(c);
        self.last_input_time = Some(now);

        let query = self.search_buffer.to_lowercase();

        let mut node = &self.root;
        for &i in &self.focus_path {
            node = &node.children[i];
        }

        let mut lines = vec![];
        self.walk(node, "".to_string(), &mut lines, self.focus_path.clone());

        if let Some(parent_path) = self
            .focus_path
            .get(..self.focus_path.len().saturating_sub(1))
        {
            let mut parent = &self.root;
            for &i in parent_path {
                parent = &parent.children[i];
            }
            for (i, sibling) in parent.children.iter().enumerate() {
                if i != *self.focus_path.last().unwrap_or(&0) {
                    lines.push((sibling.name.clone(), sibling, {
                        let mut p = parent_path.to_vec();
                        p.push(i);
                        p
                    }));
                }
            }
        }

        if let Some((_, _, path)) = lines
            .iter()
            .find(|(name, _, _)| name.to_lowercase().starts_with(&query))
        {
            self.focus_path = path.clone();
        }
    }

    fn init_terminal(&self) -> io::Result<Terminal<CrosstermBackend<Stderr>>> {
        enable_raw_mode()?;
        let mut stderr = io::stderr();
        execute!(stderr, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stderr);
        Terminal::new(backend)
    }

    fn cleanup_terminal(&self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen)?;
        Ok(())
    }
}
