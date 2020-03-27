use crate::solver::sudoku_note_entry::SudokuNoteEntry;
use crate::solver::grid::grid::Grid;

pub struct SudokuSolverStepResult {
    pub num_note_changes: usize,
    pub num_entries_fixed: usize
}

pub trait SudokuSolverStep {
    fn get_step_description(&self) -> String;

    fn do_step(&self, sudoku_notes: &mut Grid<SudokuNoteEntry>) -> SudokuSolverStepResult;
}

