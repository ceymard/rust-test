
// use std::io::{stdin};
extern crate calamine;
extern crate serde_json;

use calamine::{Sheets, Result, DataType};

use std::collections::HashMap;

/**
 * Flatten takes a HashMap that comes from our rows being derefed
 */
fn flatten(obj: &HashMap<String, DataType>) {

}

fn handle_cell(cell: &DataType) {
    match cell {
        &DataType::Float(f) => print!("{}", f),
        &DataType::Int(i) => print!("{}", i),
        &DataType::String(ref s) => print!("{}", s),
        &DataType::Bool(b) => print!("{}", b),
        &DataType::Empty => print!("empty"),
        &DataType::Error(ref e) => print!("{}", e)
    }
}

/**
 */
fn make_header(row: &[DataType]) -> Vec<String> {
    let header = row.iter().map(|ref x| x.to_string()).collect();
    header
}


fn open(pth: &str) -> Result<()> {

    let path = format!("{}", pth);
    let mut wb = Sheets::open(path)?;
    let range = wb.worksheet_range("Sheet1")?;

    let mut rows = range.rows();
    if let Some(r) = rows.next() {
        let header = make_header(r);
        println!("{:?}", header);

        let iter = rows.map(|row| {
            let mut hm = HashMap::new();

            for (str, cell) in header.iter().zip(row.iter()) {
                hm.insert(str, cell);
            }

            hm
        });

        for it in iter {
            println!("{:?}", it)
        }
    }

    // let header = make_header(rows.next());

    // for row in rows {
    //     // let x = make_header(row);
    //     println!("{:?}", row);
    // }

    Ok(())
}

fn main() {
    if let Err(e) = open("pouet.xlsx") {
        println!("Welp, couldn't open file ({}).", e);
    } else {
        println!("We opened and everything is fine.");
    }
    // let range = wb
}


#[cfg(test)]
mod other_test {

    #[test]
    fn zobi() {
        println!("I'm testing stuff.")
    }
}