use colored::Colorize;

pub struct Logger;

impl Logger {
    pub fn info(message: &str) {
        println!(
            "{} {}",
            format!(" INFO ").white().on_blue(),
            message.blue()
        );
    }

    pub fn info_bold(message: &str) {
        println!(
            "{} {}",
            format!(" INFO ").white().on_blue(),
            message.blue().bold()
        );
    }

    pub fn success(message: &str) {
        println!(
            "{} {}",
            format!(" SUCCESS ").white().on_green(),
            message.green()
        );
    }

    pub fn success_bold(message: &str) {
        println!(
            "{} {}",
            format!(" SUCCESS ").white().on_green(),
            message.green().bold()
        );
    }

    pub fn error(message: &str) {
        println!(
            "{} {}",
            format!(" ERROR ").white().on_red(),
            message.red()
        );
    }

    pub fn error_bold(message: &str) {
        println!(
            "{} {}",
            format!(" ERROR ").white().on_red(),
            message.red().bold()
        );
    }

    pub fn list(message: &str) {
        println!(
            "{} {}",
            format!(" LIST ").white().on_magenta(),
            message.magenta().bold()
        );
    }

    pub fn custom(label: &str, message: &str, bg_color: colored::Color, fg_color: colored::Color) {
        println!(
            "{} {}",
            format!(" {} ", label).white().on_color(bg_color),
            message.color(fg_color)
        );
    }

    pub fn custom_bold(label: &str, message: &str, bg_color: colored::Color, fg_color: colored::Color) {
        println!(
            "{} {}",
            format!(" {} ", label).white().on_color(bg_color),
            message.color(fg_color).bold()
        );
    }
}
