use std::{collections::HashMap, fmt::Display, fs, ops::Index};

#[derive(Debug, Clone)]
pub(crate) enum DType {
    Bool,
    Int,
    Float,
    Str,
}

#[derive(Debug, Clone)]
pub(crate) enum Cell {
    Bool(bool),
    Int(isize),
    Float(f64),
    Str(String),
}
impl Cell {
    pub(crate) fn to_string(&self) -> String {
        match self {
            Cell::Bool(v) => v.to_string(),
            Cell::Int(v) => v.to_string(),
            Cell::Float(v) => v.to_string(),
            Cell::Str(v) => v.to_string(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Header {
    name: String,
    d_type: DType,
}

#[derive(Debug, Clone)]
pub(crate) struct Series(Vec<Option<Cell>>);

impl Index<usize> for Series {
    type Output = Option<Cell>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Series {
    pub(crate) fn new(data: Vec<String>) -> Self {
        Self(
            data.iter()
                .map(|v| Some(Cell::Str(v.to_string())))
                .collect(),
        )
    }
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn push(&mut self, item: Option<Cell>) {
        self.0.push(item)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DataFrame {
    pub(crate) headers: Vec<Header>,
    pub(crate) data: HashMap<String, Series>,
}

impl Index<String> for DataFrame {
    type Output = Series;
    fn index(&self, index: String) -> &Self::Output {
        self.data
            .get(&index)
            .unwrap_or_else(|| panic!("Wrong column name provided"))
    }
}

impl DataFrame {
    pub(crate) fn new() -> Self {
        Self {
            headers: vec![],
            data: HashMap::new(),
        }
    }
    pub(crate) fn shape(&self) -> (usize, usize) {
        let first_header = self.headers.get(0).unwrap().name.clone();

        match self.get(first_header) {
            Some(s) => (s.len(), self.headers.len()),
            None => (0, self.headers.len()),
        }
    }
    pub(crate) fn from_mapping(data: HashMap<String, Vec<String>>) -> Self {
        let headers = (&data)
            .keys()
            .into_iter()
            .map(|k| Header {
                d_type: DType::Str,
                name: k.to_string(),
            })
            .collect();
        Self {
            headers: headers,
            data: data
                .iter()
                .map(|(k, v)| (k.to_string(), Series::new(v.to_owned())))
                .collect(),
        }
    }
    pub(crate) fn push(&mut self, row: Vec<Option<Cell>>) {
        if row.len() != self.headers.len() {
            panic!("Row dimension and header dimension mismatch")
        }
        self.headers.iter().zip(row).for_each(|(header, item)| {
            match self.data.get_mut(&header.name) {
                Some(s) => {
                    // println!("Series: {:?}", s);
                    s.push(item);
                }
                None => println!("Header: {}, value: {:?}", header.name, item),
            };
        });
    }

    fn get(&self, title: String) -> Option<&Series> {
        self.data.get(&title)
    }

    pub(crate) fn get_item_at(&self, index: usize) -> Vec<Option<Cell>> {
        self.headers
            .iter()
            .map(|header| self.data.get(&header.name).unwrap()[index].clone())
            .collect()
    }

    pub(crate) fn describe(&self) {
        self.headers.iter().for_each(|item| {
            println!("{:<30}: {:?}", item.name, item.d_type);
        });
    }
    pub(crate) fn head(&self, mut n: usize) {
        if n > self.shape().0 {
            n = self.shape().0
        }
        let titles = self
            .headers
            .iter()
            .map(|h| format!("{:^20}", h.name))
            .collect::<Vec<String>>()
            .join(" | ");
        println!("{titles}");
        println!("{}", "-".repeat(titles.len()));
        for idx in 0..n {
            let row = self
                .get_item_at(idx)
                .iter()
                .map(|_cell| match _cell {
                    Some(cell) => format!("{:<20}", cell.to_string()),
                    None => String::from(" ".repeat(20)),
                })
                .collect::<Vec<String>>()
                .join(" | ");

            println!("{}", row);
        }
    }
}

pub(crate) fn read_csv(path: &str, headers: bool) -> DataFrame {
    let mut lines: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|v| v.to_string())
        .collect();

    let mut df = DataFrame::new();
    let mut df = DataFrame::from_mapping(
        lines
            .get(0)
            .unwrap()
            .split(",")
            .enumerate()
            .map(|(idx, name)| match headers {
                true => (name.trim().to_string(), vec![]),
                false => (idx.to_string(), vec![]),
            })
            .collect(),
    );
    if headers {
        lines.remove(0);
    }
    lines.iter().for_each(|line| {
        let row = line
            .split(",")
            .map(|v| Some(Cell::Str(v.trim().to_string())))
            .collect::<Vec<Option<Cell>>>();
        df.push(row)
    });
    df
}
