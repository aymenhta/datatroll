use super::*;

#[test]
fn test_data_loading() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    let want = vec![
        vec![
            Cell::String("id".to_string()),
            Cell::String("title".to_string()),
            Cell::String("director".to_string()),
            Cell::String("release date".to_string()),
            Cell::String("review".to_string()),
            Cell::String("overrated".to_string()),
        ],
        vec![
            Cell::Int(1),
            Cell::String("old".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2011),
            Cell::Float(3.5),
            Cell::Bool(true),
        ],
        vec![
            Cell::Int(2),
            Cell::String("her".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2013),
            Cell::Float(4.2),
            Cell::Bool(true),
        ],
        vec![
            Cell::Int(3),
            Cell::String("easy".to_string()),
            Cell::String("scorces".to_string()),
            Cell::Int(2005),
            Cell::Float(1.0),
            Cell::Bool(false),
        ],
        vec![
            Cell::Int(4),
            Cell::String("hey".to_string()),
            Cell::String("nolan".to_string()),
            Cell::Int(1997),
            Cell::Float(4.7),
            Cell::Bool(true),
        ],
        vec![
            Cell::Int(5),
            Cell::String("who".to_string()),
            Cell::String("martin".to_string()),
            Cell::Int(2017),
            Cell::Float(5.0),
            Cell::Bool(false),
        ],
    ];

    for i in 1..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i])
    }
}

#[test]
fn test_data_loading_should_return_err() {
    let mut sheet = Sheet::new_sheet();
    assert!(sheet.load_data("non_existent.csv").is_err());
}

#[test]
fn test_mean() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    assert_eq!(sheet.mean("review").unwrap(), 3.72)
}

#[test]
fn test_median() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    assert_eq!(*sheet.median("release date"), Cell::Int(2005))
}

#[test]
fn test_mode() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("freq_data.csv").unwrap();

    let got = sheet.mode("director");
    let want = (Cell::String("scorces".to_string()), 3);
    assert_eq!(got, want)
}

#[test]
fn test_max_int64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    assert_eq!(sheet.max_int64("release date").unwrap(), 2017)
}

#[test]
fn test_max_float64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    assert_eq!(sheet.max_float64("review").unwrap(), 5.0)
}

#[test]
fn test_min_int64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    assert_eq!(sheet.min_int64("release date").unwrap(), 1997)
}

#[test]
fn test_min_float64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    assert_eq!(sheet.min_float64("review").unwrap(), 1.2)
}

#[test]
fn test_insert() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    sheet
        .insert_row("7, hello, quintin, 2007, 2.4, true")
        .unwrap();
    let want = vec![
        Cell::Int(7),
        Cell::String("hello".to_string()),
        Cell::String("quintin".to_string()),
        Cell::Int(2007),
        Cell::Float(2.4),
        Cell::Bool(true),
    ];
    let got = sheet.data.last().unwrap();

    assert_sheet_row(&got, &want)
}

#[test]
fn test_drop_rows() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    sheet.drop_rows("review", |c| {
        if let Cell::Float(r) = c {
            if *r < 4.0 {
                return true;
            }
        }
        false
    });

    let want = vec![
        vec![
            Cell::String("id".to_string()),
            Cell::String("title".to_string()),
            Cell::String("director".to_string()),
            Cell::String("release date".to_string()),
            Cell::String("review".to_string()),
            Cell::String("overrated".to_string()),
        ],
        vec![
            Cell::Int(2),
            Cell::String("her".to_string()),
            Cell::String("quintin".to_string()),
            Cell::Int(2013),
            Cell::Float(4.2),
            Cell::Bool(true),
        ],
        vec![
            Cell::Int(4),
            Cell::String("hey".to_string()),
            Cell::String("nolan".to_string()),
            Cell::Int(1997),
            Cell::Float(4.7),
            Cell::Bool(true),
        ],
        vec![
            Cell::Int(5),
            Cell::String("who".to_string()),
            Cell::String("martin".to_string()),
            Cell::Int(2017),
            Cell::Float(5.0),
            Cell::Bool(false),
        ],
    ];

    for i in 1..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i]);
    }
}

#[test]
fn test_drop_col() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    sheet.drop_col("overrated");

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

    for i in 1..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i])
    }
}

#[test]
fn test_variance() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

    let got = sheet.variance("review").unwrap();
    let want = 1.8456000000000004;
    assert_eq!(got, want)
}

#[test]
fn test_find_first_row() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("test_data.csv").unwrap();

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

fn assert_sheet_row(got: &Vec<Cell>, want: &Vec<Cell>) {
    assert_eq!(got.len(), want.len());

    let id = match got[0] {
        Cell::Int(i) => i,
        _ => 0,
    };

    let title = match &got[1] {
        Cell::String(s) => s,
        _ => "",
    };

    let director = match &got[2] {
        Cell::String(s) => s,
        _ => "",
    };

    let release_date = match got[0] {
        Cell::Int(i) => i,
        _ => 0,
    };

    let review = match got[0] {
        Cell::Float(f) => f,
        _ => 0_f64,
    };

    let want_id = match want[0] {
        Cell::Int(i) => i,
        _ => 0,
    };

    let want_title = match &want[1] {
        Cell::String(s) => s,
        _ => "",
    };

    let want_director = match &want[2] {
        Cell::String(s) => s,
        _ => "",
    };

    let want_release_date = match want[0] {
        Cell::Int(i) => i,
        _ => 0,
    };

    let want_review = match want[0] {
        Cell::Float(f) => f,
        _ => 0_f64,
    };

    assert_eq!(id, want_id);
    assert_eq!(title, want_title);
    assert_eq!(director, want_director);
    assert_eq!(release_date, want_release_date);
    assert_eq!(review, want_review);
}
