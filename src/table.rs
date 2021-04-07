use prettytable::{Table, Row};

pub trait TableDisplay {

    // Produce the header of the table for this type
    fn header(&self) -> Row;

    // Produce the remaining rows for the table. 
    fn rows(&self) -> Vec<Row>;
}

pub fn display<T: TableDisplay>(td: &T) {
    // Create a base table
    let mut table = Table::new();

    // Add the header
    table.add_row(td.header());

    for row in td.rows() {
        table.add_row(row);
    }

    table.printstd();
}