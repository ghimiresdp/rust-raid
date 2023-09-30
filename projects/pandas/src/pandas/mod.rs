use std::{collections::HashMap, fmt::Display, fs};

#[derive(Debug)]
pub(crate) enum DType {
    Bool,
    Int,
    Float,
    String,
    Csv,
    Map,
}

#[derive(Debug)]
pub(crate) enum Cell {
    Bool(bool),
    Int(isize),
    Float(f64),
    Text(String),
    Csv(Vec<Cell>),
    Map(HashMap<String, Cell>),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Bool(v) => format!("{}", v),
                Cell::Int(v) => format!("{}", v),
                Cell::Float(v) => format!("{}", v),
                Cell::Text(v) => format!("{}", v),
                Cell::Csv(v) => format!("CSV"),
                Cell::Map(_) => format!("MAP"),
            }
        )
    }
}

#[derive(Debug)]
pub(crate) struct Header {
    name: String,
    d_type: DType,
}

#[derive(Debug)]
pub(crate) struct DataFrame {
    pub(crate) headers: Vec<Header>,
    pub(crate) shape: (usize, usize),
    pub(crate) data: HashMap<String, Vec<Option<Cell>>>,
}

impl DataFrame {
    pub(crate) fn new() -> Self {
        Self {
            headers: vec![],
            shape: (0, 0), // (row, col)
            data: HashMap::new(),
        }
    }
    pub(crate) fn describe(&self) {
        self.headers.iter().for_each(|item| {
            println!("{:<30}: {:?}", item.name, item.d_type);
        });
    }
    pub(crate) fn head(&self, mut n: usize) {
        if n > self.shape.0 {
            n = self.shape.0
        }
        let titles = self
            .headers
            .iter()
            .map(|h| format!("{:^20}", h.name))
            .collect::<Vec<String>>()
            .join("|");
        println!("{titles}");
        println!("{}", "-".repeat(titles.len()));
        // for idx in 0..n {
        //     let x = self
        //         .headers
        //         .iter()
        //         .map(|h| {
        //             let entry = self.data.get(h.name).unwrap().clone().get(idx).unwrap();
        //         })
        //         .collect();
        // }
        // self.rows[0..n].iter().for_each(|r| {
        //     println!(
        //         "{}",
        //         r.iter()
        //             .map(|v| { format!("{:?}", v) })
        //             .collect::<Vec<String>>()
        //             .join("|")
        //     );
        // });
    }
}

pub(crate) fn read_csv(path: &str, headers: bool) -> DataFrame {
    let mut lines: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|v| v.to_string())
        .collect();

    let mut df = DataFrame::new();
    df.headers = lines
        .get(0)
        .unwrap()
        .split(",")
        .enumerate()
        .map(|(idx, name)| Header {
            name: match headers {
                true => name.trim().to_string(),
                false => idx.to_string(),
            },
            d_type: DType::String,
        })
        .collect();
    if headers {
        lines.remove(0);
    }
    // df.headers.iter().for_each(|h| {
    //     df.data.insert(h.name, vec![]);
    // });
    // lines.iter().for_each(|line| {
    //     line.split(",").enumerate().for_each(|(idx, value)| {
    //         df.data
    //             .entry(df.headers[idx].name)
    //             .and_modify(|v| v.push(Some(Cell::Text(value.trim().to_string()))));
    //     })
    // });
    df.shape = (lines.len(), df.headers.len());
    df
}
