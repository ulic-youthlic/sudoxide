// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::{Arc, Mutex};
use sudoku::{
    generator::{
        random_sudoku_puzzle_easy, random_sudoku_puzzle_extraeasy, random_sudoku_puzzle_extrahard,
        random_sudoku_puzzle_hard, random_sudoku_puzzle_normal, random_sudoku_puzzle_ultimate,
    },
    judge::judge_sudoku as judge,
};
use tauri::State;

fn main() {
    tauri::Builder::default()
        .manage(SettingsState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            get_sudoku_puzzle,
            judge_sudoku,
            set_difficulty,
            get_difficulty,
            set_marking_assist,
            get_marking_assist,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
struct Settings {
    difficulty: u8,
    marking_assist: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            difficulty: 2,
            marking_assist: false,
        }
    }
}

#[derive(Default)]
struct SettingsState(Arc<Mutex<Settings>>);

#[tauri::command]
async fn get_sudoku_puzzle(settings: State<'_, SettingsState>) -> Result<[[i8; 9]; 9], ()> {
    let difficulty = settings.0.lock().unwrap().difficulty;
    Ok(match difficulty {
        0 => random_sudoku_puzzle_extraeasy().0,
        1 => random_sudoku_puzzle_easy().0,
        2 => random_sudoku_puzzle_normal().0,
        3 => random_sudoku_puzzle_hard().0,
        4 => random_sudoku_puzzle_extrahard().0,
        _ => random_sudoku_puzzle_ultimate().0,
    })
}

#[tauri::command]
async fn judge_sudoku(board: [[i8; 9]; 9]) -> (bool, [[bool; 9]; 9]) {
    let res = judge(&sudoku::Grid(board));
    (res.1, res.2)
}

#[tauri::command]
fn set_difficulty(new_difficulty: u8, settings: State<'_, SettingsState>) {
    let new_difficulty = if new_difficulty > 5 {
        5
    } else {
        new_difficulty
    };
    settings.0.lock().unwrap().difficulty = new_difficulty;
}

#[tauri::command]
fn get_difficulty(settings: State<'_, SettingsState>) -> Result<u8, ()> {
    Ok(settings.0.lock().unwrap().difficulty)
}

#[tauri::command]
fn set_marking_assist(marking_assist: bool, settings: State<'_, SettingsState>) {
    settings.0.lock().unwrap().marking_assist = marking_assist;
}

#[tauri::command]
fn get_marking_assist(settings: State<'_, SettingsState>) -> Result<bool, ()> {
    Ok(settings.0.lock().unwrap().marking_assist)
}
