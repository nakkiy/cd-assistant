use crate::tree::DirEntry;
use chrono::{DateTime, Local};
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem},
};
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub struct Popup {
    pub visible: bool,
    offset: usize,
}

impl Popup {
    pub fn new() -> Self {
        Self {
            visible: false,
            offset: 0,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if !self.visible {
            self.offset = 0;
        }
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.offset = 0;
    }

    pub fn scroll_up(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }
    }

    pub fn scroll_down(&mut self, max: usize, view_height: usize) {
        if self.offset + view_height < max {
            self.offset += 1;
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, node: &DirEntry) {
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };
        f.render_widget(Clear, popup_area);

        let block = Block::default()
            .title("file list")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let file_items: Vec<ListItem> = match fs::read_dir(&node.path) {
            Ok(entries) => {
                let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                files.sort_by_key(|e| e.file_name());

                files
                    .into_iter()
                    .filter_map(|e| Self::format_file_item(&e))
                    .collect()
            }
            Err(_) => vec![ListItem::new("reading error")],
        };

        let view_height = popup_area.height.saturating_sub(2) as usize;
        let max_offset = file_items.len().saturating_sub(view_height);
        let offset = self.offset.min(max_offset);
        let visible_items = file_items
            .into_iter()
            .skip(offset)
            .take(view_height)
            .collect::<Vec<_>>();

        let file_list = List::new(visible_items).block(block);
        f.render_widget(file_list, popup_area);
    }

    fn format_file_item(entry: &fs::DirEntry) -> Option<ListItem<'static>> {
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let meta = fs::symlink_metadata(&path).ok()?;

        if meta.is_dir() {
            return None;
        }

        let (display_name, style) = if meta.file_type().is_symlink() {
            let target = path.read_link().ok();
            match fs::metadata(&path) {
                Ok(target_meta) => {
                    if target_meta.is_dir() {
                        return None;
                    }
                    let link_str = target
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "???".into());
                    (
                        format!("{} -> {}", file_name, link_str),
                        Style::default().fg(Color::Cyan),
                    )
                }
                Err(_) => (
                    format!("{} -> ???", file_name),
                    Style::default().fg(Color::Red),
                ),
            }
        } else if meta.is_file() {
            let style = if meta.permissions().mode() & 0o111 != 0 {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            (file_name, style)
        } else {
            return None;
        };

        let size = meta.len();
        let modified = meta
            .modified()
            .ok()
            .map(|mtime| {
                let datetime: DateTime<Local> = mtime.into();
                datetime.format("%Y-%m-%d %H:%M").to_string()
            })
            .unwrap_or_else(|| "???".into());

        let mode = meta.permissions().mode();
        let perms = format!(
            "{}{}{}{}{}{}{}{}{}",
            if mode & 0o400 != 0 { "r" } else { "-" },
            if mode & 0o200 != 0 { "w" } else { "-" },
            if mode & 0o100 != 0 { "x" } else { "-" },
            if mode & 0o040 != 0 { "r" } else { "-" },
            if mode & 0o020 != 0 { "w" } else { "-" },
            if mode & 0o010 != 0 { "x" } else { "-" },
            if mode & 0o004 != 0 { "r" } else { "-" },
            if mode & 0o002 != 0 { "w" } else { "-" },
            if mode & 0o001 != 0 { "x" } else { "-" },
        );

        let info = format!(
            "{:<30} {:>8} {:<16} {}",
            display_name,
            format_size(size),
            modified,
            perms
        );

        Some(ListItem::new(info).style(style))
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
