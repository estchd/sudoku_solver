pub struct Sector<'a, EntryType> {
    sector_entries: Vec<Vec<&'a EntryType>>
}

impl<'a, EntryType> Sector<'a, EntryType> {
    pub fn new(entries: Vec<Vec<&'a EntryType>>) -> Sector<'a, EntryType> {
        return Sector {
            sector_entries: entries
        }
    }

    pub fn get_entry(&self, x: usize, y: usize) {

    }
}