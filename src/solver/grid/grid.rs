use std::collections::hash_map::Entry;

pub struct Grid<EntryType> {
    column_count: usize,
    row_count: usize,
    entries: Vec<EntryType>
}

impl<EntryType> Grid<EntryType> {
    pub fn new_from_array(column_count: usize, row_count: usize, entries: Vec<EntryType>) -> Result<Grid<EntryType>,()> {
        if entries.len() != column_count*row_count {
            return Err(());
        }
        return Ok(Grid {
            column_count,
            row_count,
            entries
        });
    }

    pub fn get_entry(&self, x: usize, y: usize) -> Result<&EntryType,()> {
        if x >= self.column_count {
            return Err(())
        }
        if y >= self.row_count {
            return Err(())
        }
        return Ok(&self.entries[(y*self.column_count) + x]);
    }

    pub fn get_entry_mut(&mut self, x: usize, y: usize) -> Result<&mut EntryType,()> {
        if x >= self.column_count {
            return Err(())
        }
        if y >= self.row_count {
            return Err(())
        }
        return Ok(&mut self.entries[(y*self.column_count) + x]);
    }

    pub fn get_row(&self, y: usize) -> Result<(),()> {

    }
}

impl<EntryType: Default> Grid<EntryType> {
    pub fn new(column_count: usize, row_count: usize) -> Grid<EntryType> {
        let mut entries = Vec::<EntryType>::new();
        entries.reserve(column_count*row_count);
        for i in 0..column_count*row_count {
            entries.push(EntryType::default())
        }
        return Grid {
            column_count,
            row_count,
            entries,
        }
    }
}

struct Row<'a, EntryType> {
    row_index: usize,
    parent_grid: &'a Grid<EntryType>
}

impl<'a, EntryType> Row<'a, EntryType> {
    pub fn new(row_index: usize, parent_grid: &'a Grid<EntryType>) -> Row<'a, EntryType> {
        return Row {
            row_index,
            parent_grid,
        }
    }

    pub fn get_entry(&self, index: usize) -> Result<&EntryType,()> {
        return self.parent_grid.get_entry(index, self.row_index);
    }
}

struct MutRow<'a, EntryType> {
    row_index: usize,
    parent_grid: &'a mut Grid<EntryType>
}

impl<'a, EntryType> MutRow<'a, EntryType> {
    pub fn new(row_index: usize, parent_grid: &'a mut Grid<EntryType>) -> MutRow<'a, EntryType> {
        return MutRow {
            row_index,
            parent_grid,
        }
    }

    pub fn get_entry(&self, index: usize) -> Result<&EntryType,()> {
        return self.parent_grid.get_entry(index, self.row_index);
    }

    pub fn get_entry_mut(&mut self, index: usize) -> Result<&mut EntryType, ()> {
        return self.parent_grid.get_entry_mut(index, self.row_index);
    }
}

struct Column<'a, EntryType> {
    column_index: usize,
    parent_grid: &'a mut Grid<EntryType>
}

impl<'a, EntryType> Column<'a, EntryType> {
    pub fn new(column_index: usize, parent_grid: &'a Grid<EntryType>) ->Column<'a, EntryType> {
        return Column {
            column_index,
            parent_grid,
        }
    }

    pub fn get_entry(&self, index: usize) -> Result<&EntryType,()> {
        return self.parent_grid.get_entry(self.column_index, index);
    }
}

struct MutColumn<'a, EntryType> {
    column_index: usize,
    parent_grid: &'a mut Grid<EntryType>
}

impl<'a, EntryType> MutColumn<'a, EntryType> {
    pub fn new(column_index: usize, parent_grid: &'a mut Grid<EntryType>) -> MutColumn<'a, EntryType> {
        return MutColumn {
            column_index,
            parent_grid,
        }
    }

    pub fn get_entry(&self, index: usize) -> Result<&EntryType,()> {
        return self.parent_grid.get_entry(self.column_index, index);
    }

    pub fn get_entry_mut(&mut self, index: usize) -> Result<&mut EntryType, ()> {
        return self.parent_grid.get_entry_mut(self.column_index, index);
    }
}