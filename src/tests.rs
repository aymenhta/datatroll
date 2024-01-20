use super::*;

#[test]
fn test_data_loading() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    let want = vec![
        vec![
            CellType::StringCell("id".to_string()),
            CellType::StringCell("title".to_string()),
            CellType::StringCell("director".to_string()),
            CellType::StringCell("release date".to_string()),
            CellType::StringCell("review".to_string()),
            CellType::StringCell("overrated".to_string()),
        ],
        vec![
            CellType::IntCell(1),
            CellType::StringCell("old".to_string()),
            CellType::StringCell("quintin".to_string()),
            CellType::IntCell(2011),
            CellType::FloatCell(3.5),
            CellType::BooleanCell(true),
        ],
        vec![
            CellType::IntCell(2),
            CellType::StringCell("her".to_string()),
            CellType::StringCell("quintin".to_string()),
            CellType::IntCell(2013),
            CellType::FloatCell(4.2),
            CellType::BooleanCell(true),
        ],
        vec![
            CellType::IntCell(3),
            CellType::StringCell("easy".to_string()),
            CellType::StringCell("scorces".to_string()),
            CellType::IntCell(2005),
            CellType::FloatCell(1.0),
            CellType::BooleanCell(false),
        ],
        vec![
            CellType::IntCell(4),
            CellType::StringCell("hey".to_string()),
            CellType::StringCell("nolan".to_string()),
            CellType::IntCell(1997),
            CellType::FloatCell(4.7),
            CellType::BooleanCell(true),
        ],
        vec![
            CellType::IntCell(5),
            CellType::StringCell("who".to_string()),
            CellType::StringCell("martin".to_string()),
            CellType::IntCell(2017),
            CellType::FloatCell(5.0),
            CellType::BooleanCell(false),
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
    sheet.load_data("data.csv").unwrap();

    assert_eq!(sheet.mean("review").unwrap(), 3.72)
}

#[test]
fn test_median() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    assert_eq!(sheet.median("release date"), CellType::IntCell(2005))
}

#[test]
fn test_mode() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("freq_data.csv").unwrap();

    let got = sheet.mode("director");
    let want = (CellType::StringCell("scorces".to_string()), 3);
    assert_eq!(got, want)
}

#[test]
fn test_max_int64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    assert_eq!(sheet.max_int64("release date").unwrap(), 2017)
}

#[test]
fn test_max_float64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    assert_eq!(sheet.max_float64("review").unwrap(), 5.0)
}

#[test]
fn test_min_int64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    assert_eq!(sheet.min_int64("release date").unwrap(), 1997)
}

#[test]
fn test_min_float64() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    assert_eq!(sheet.min_float64("review").unwrap(), 1.2)
}

#[test]
fn test_insert() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    sheet
        .insert_row("7, hello, quintin, 2007, 2.4, true")
        .unwrap();
    let want = vec![
        CellType::IntCell(7),
        CellType::StringCell("hello".to_string()),
        CellType::StringCell("quintin".to_string()),
        CellType::IntCell(2007),
        CellType::FloatCell(2.4),
        CellType::BooleanCell(true),
    ];
    let got = sheet.data.last().unwrap();

    assert_sheet_row(&got, &want)
}

#[test]
fn test_drop_rows() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    sheet.drop_rows("review", |c| {
        if let CellType::FloatCell(r) = c {
            if *r < 4.0 {
                return true;
            }
        }
        false
    });

    let want = vec![
        vec![
            CellType::StringCell("id".to_string()),
            CellType::StringCell("title".to_string()),
            CellType::StringCell("director".to_string()),
            CellType::StringCell("release date".to_string()),
            CellType::StringCell("review".to_string()),
            CellType::StringCell("overrated".to_string()),
        ],
        vec![
            CellType::IntCell(2),
            CellType::StringCell("her".to_string()),
            CellType::StringCell("quintin".to_string()),
            CellType::IntCell(2013),
            CellType::FloatCell(4.2),
            CellType::BooleanCell(true),
        ],
        vec![
            CellType::IntCell(4),
            CellType::StringCell("hey".to_string()),
            CellType::StringCell("nolan".to_string()),
            CellType::IntCell(1997),
            CellType::FloatCell(4.7),
            CellType::BooleanCell(true),
        ],
        vec![
            CellType::IntCell(5),
            CellType::StringCell("who".to_string()),
            CellType::StringCell("martin".to_string()),
            CellType::IntCell(2017),
            CellType::FloatCell(5.0),
            CellType::BooleanCell(false),
        ],
    ];

    for i in 1..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i]);
    }
}

#[test]
fn test_drop_col() {
    let mut sheet = Sheet::new_sheet();
    sheet.load_data("data.csv").unwrap();

    sheet.drop_col("overrated");

    let want = vec![
        vec![
            CellType::StringCell("id".to_string()),
            CellType::StringCell("title".to_string()),
            CellType::StringCell("director".to_string()),
            CellType::StringCell("release date".to_string()),
            CellType::StringCell("review".to_string()),
        ],
        vec![
            CellType::IntCell(1),
            CellType::StringCell("old".to_string()),
            CellType::StringCell("quintin".to_string()),
            CellType::IntCell(2011),
            CellType::FloatCell(3.5),
        ],
        vec![
            CellType::IntCell(2),
            CellType::StringCell("her".to_string()),
            CellType::StringCell("quintin".to_string()),
            CellType::IntCell(2013),
            CellType::FloatCell(4.2),
        ],
        vec![
            CellType::IntCell(3),
            CellType::StringCell("easy".to_string()),
            CellType::StringCell("scorces".to_string()),
            CellType::IntCell(2005),
            CellType::FloatCell(1.0),
        ],
        vec![
            CellType::IntCell(4),
            CellType::StringCell("hey".to_string()),
            CellType::StringCell("nolan".to_string()),
            CellType::IntCell(1997),
            CellType::FloatCell(4.7),
        ],
        vec![
            CellType::IntCell(5),
            CellType::StringCell("who".to_string()),
            CellType::StringCell("martin".to_string()),
            CellType::IntCell(2017),
            CellType::FloatCell(5.0),
        ],
    ];

    for i in 1..sheet.data.len() {
        assert_sheet_row(&sheet.data[i], &want[i])
    }
}

fn assert_sheet_row(got: &Vec<CellType>, want: &Vec<CellType>) {
    assert_eq!(got.len(), want.len());

    let id = match got[0] {
        CellType::IntCell(i) => i,
        _ => 0,
    };

    let title = match &got[1] {
        CellType::StringCell(s) => s,
        _ => "",
    };

    let director = match &got[2] {
        CellType::StringCell(s) => s,
        _ => "",
    };

    let release_date = match got[0] {
        CellType::IntCell(i) => i,
        _ => 0,
    };

    let review = match got[0] {
        CellType::FloatCell(f) => f,
        _ => 0_f64,
    };

    let want_id = match want[0] {
        CellType::IntCell(i) => i,
        _ => 0,
    };

    let want_title = match &want[1] {
        CellType::StringCell(s) => s,
        _ => "",
    };

    let want_director = match &want[2] {
        CellType::StringCell(s) => s,
        _ => "",
    };

    let want_release_date = match want[0] {
        CellType::IntCell(i) => i,
        _ => 0,
    };

    let want_review = match want[0] {
        CellType::FloatCell(f) => f,
        _ => 0_f64,
    };

    assert_eq!(id, want_id);
    assert_eq!(title, want_title);
    assert_eq!(director, want_director);
    assert_eq!(release_date, want_release_date);
    assert_eq!(review, want_review);
}
