use super::{Cell, Sheet};

const STR_DATA: &str = "id ,title , director, release date, review
1, old, quintin, 2011, 3.5
2, her, quintin, 2013, 4.2
3, easy, scorces, 2005, 1.0
4, hey, nolan, 1997, 4.7
5, who, martin, 2017, 5.0";

#[test]
fn test_data_loading() {
    let sheet = Sheet::load_data("test_data.csv").unwrap();

    let want = vec![
        vec![
            Cell::String("id".to_string()),
            Cell::String("title".to_string()),
            Cell::String("director".to_string()),
            Cell::String("release date".to_string()),
            Cell::String("review".to_string()),
        ],
        vec![
            Cell::Int(1),
            Cell::String("old".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2011),
            Cell::Float(3.5),
        ],
        vec![
            Cell::Int(2),
            Cell::String("her".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2013),
            Cell::Float(4.2),
        ],
        vec![
            Cell::Int(3),
            Cell::String("easy".to_string()),
            Cell::String("scorces".to_string()),
            Cell::Int(2005),
            Cell::Float(1.0),
        ],
        vec![
            Cell::Int(4),
            Cell::String("hey".to_string()),
            Cell::String("nolan".to_string()),
            Cell::Int(1997),
            Cell::Float(4.7),
        ],
        vec![
            Cell::Int(5),
            Cell::String("who".to_string()),
            Cell::String("martin".to_string()),
            Cell::Int(2017),
            Cell::Float(5.0),
        ],
    ];

    for i in 0..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i])
    }
}

#[test]
fn test_data_loading_should_return_err() {
    assert!(Sheet::load_data("non_existent.csv").is_err());
}

#[test]
fn test_mean() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    assert_eq!(sheet.mean("review").unwrap(), 3.6799999999999997)
}

#[test]
fn test_median() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    assert_eq!(*sheet.median("release date"), Cell::Int(2005))
}

#[test]
fn test_mode() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    let got = &sheet.mode("director")[0];
    let want = (Cell::String("quintin".to_string()), 2);
    assert_eq!(*got, want)
}

#[test]
fn test_max_int64() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    assert_eq!(sheet.max_int64("release date").unwrap(), 2017)
}

#[test]
fn test_max_float64() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    assert_eq!(sheet.max_float64("review").unwrap(), 5.0)
}

#[test]
fn test_min_int64() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    assert_eq!(sheet.min_int64("release date").unwrap(), 1997)
}

#[test]
fn test_min_float64() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    assert_eq!(sheet.min_float64("review").unwrap(), 1.0)
}

#[test]
fn test_insert() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    sheet.insert_row("7, hello, quintin, 2007, 2.4").unwrap();
    let want = vec![
        Cell::Int(7),
        Cell::String("hello".to_string()),
        Cell::String("quintin".to_string()),
        Cell::Int(2007),
        Cell::Float(2.4),
    ];
    let got = sheet.data.last().unwrap();

    assert_sheet_row(&got, &want)
}

#[test]
fn test_drop_rows() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    sheet.drop_rows("review", |c| match c {
        Cell::Float(r) => *r < 4.0,
        _ => false,
    });

    let want = vec![
        vec![
            Cell::String("id".to_string()),
            Cell::String("title".to_string()),
            Cell::String("director".to_string()),
            Cell::String("release date".to_string()),
            Cell::String("review".to_string()),
        ],
        vec![
            Cell::Int(2),
            Cell::String("her".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2013),
            Cell::Float(4.2),
        ],
        vec![
            Cell::Int(4),
            Cell::String("hey".to_string()),
            Cell::String("nolan".to_string()),
            Cell::Int(1997),
            Cell::Float(4.7),
        ],
        vec![
            Cell::Int(5),
            Cell::String("who".to_string()),
            Cell::String("martin".to_string()),
            Cell::Int(2017),
            Cell::Float(5.0),
        ],
    ];

    for i in 0..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i]);
    }
}

#[test]
fn test_drop_col() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    sheet.drop_col("review");

    let want = vec![
        vec![
            Cell::String("id".to_string()),
            Cell::String("title".to_string()),
            Cell::String("director".to_string()),
            Cell::String("release date".to_string()),
        ],
        vec![
            Cell::Int(1),
            Cell::String("old".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2011),
        ],
        vec![
            Cell::Int(2),
            Cell::String("her".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2013),
        ],
        vec![
            Cell::Int(3),
            Cell::String("easy".to_string()),
            Cell::String("scorces".to_string()),
            Cell::Int(2005),
        ],
        vec![
            Cell::Int(4),
            Cell::String("hey".to_string()),
            Cell::String("nolan".to_string()),
            Cell::Int(1997),
        ],
        vec![
            Cell::Int(5),
            Cell::String("who".to_string()),
            Cell::String("martin".to_string()),
            Cell::Int(2017),
        ],
    ];

    for i in 1..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i])
    }
}

#[test]
fn test_fill_col() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    sheet.fill_col("id", Cell::Null).unwrap();
    for row in sheet.paginate(1, sheet.data.len() - 1).unwrap() {
        println!("{:?}", row[1]);
        assert_eq!(Cell::Null, row[0]);
    }
}

#[test]
fn test_variance() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    let got = sheet.variance("review").unwrap();
    let want = 2.0536000000000003;
    assert_eq!(got, want)
}

#[test]
fn test_map() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    let _ = sheet.map("title", |c| match c {
        Cell::String(s) => Cell::String(s.to_uppercase()),
        _ => return c,
    });

    let want = vec![
        Cell::String("TITLE".to_string()),
        Cell::String("OLD".to_string()),
        Cell::String("HER".to_string()),
        Cell::String("EASY".to_string()),
        Cell::String("HEY".to_string()),
        Cell::String("WHO".to_string()),
    ];

    for i in 0..sheet.data.len() {
        assert_eq!(&sheet.data[i][1], &want[i])
    }
}

#[test]
fn test_map_fails_when_col_doesnot_exist() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    assert!(sheet
        .map("overrated", |c| match c {
            Cell::String(s) => Cell::String(s.to_uppercase()),
            _ => return c,
        })
        .is_err());
}

#[test]
fn test_find_first_row() {
    let sheet = Sheet::load_data_from_str(STR_DATA);

    let got = sheet.find_first_row("review", |c| {
        if let Cell::Float(r) = c {
            if *r > 4.0 {
                return true;
            }
        }
        false
    });

    let got2 = sheet.find_first_row("id", |c| {
        if let Cell::Int(i) = c {
            if *i > 10 {
                return true;
            }
        }
        false
    });

    assert!(got.is_some());
    assert!(got2.is_none());
}

#[test]
fn test_edit_cell() {
    let mut sheet = Sheet::load_data_from_str(STR_DATA);

    let (_, i) = sheet
        .find_first_row("release date", |c| match c {
            Cell::Int(i) => *i == 2013,
            _ => false,
        })
        .unwrap();

    sheet.edit_cell("release date", i, Cell::Int(2022)).unwrap();
    assert_eq!(sheet.data[i][3], Cell::Int(2022));
}

fn assert_sheet_row(got: &Vec<Cell>, want: &Vec<Cell>) {
    assert_eq!(got.len(), want.len());

    for i in 0..got.len() {
        assert_eq!(got[i], want[i])
    }
}
