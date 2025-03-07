use chrono::{DateTime, Local};
use nu_test_support::fs::Stub;
use nu_test_support::nu;
use nu_test_support::playground::Playground;

#[test]
fn creates_a_file_when_it_doesnt_exist() {
    Playground::setup("create_test_1", |dirs, _sandbox| {
        nu!(
            cwd: dirs.test(),
            "touch i_will_be_created.txt"
        );

        let path = dirs.test().join("i_will_be_created.txt");
        assert!(path.exists());
    })
}

#[test]
fn creates_two_files() {
    Playground::setup("create_test_2", |dirs, _sandbox| {
        nu!(
            cwd: dirs.test(),
            "touch a b"
        );

        let path = dirs.test().join("a");
        assert!(path.exists());

        let path2 = dirs.test().join("b");
        assert!(path2.exists());
    })
}

#[test]
fn change_modified_time_of_file_to_today() {
    Playground::setup("change_time_test_9", |dirs, sandbox| {
        sandbox.with_files(vec![Stub::EmptyFile("file.txt")]);

        nu!(
            cwd: dirs.test(),
            "touch -m file.txt"
        );

        let path = dirs.test().join("file.txt");

        // Check only the date since the time may not match exactly
        let date = Local::now().date_naive();
        let actual_date_time: DateTime<Local> =
            DateTime::from(path.metadata().unwrap().modified().unwrap());
        let actual_date = actual_date_time.date_naive();

        assert_eq!(date, actual_date);
    })
}

#[test]
fn change_access_time_of_file_to_today() {
    Playground::setup("change_time_test_18", |dirs, sandbox| {
        sandbox.with_files(vec![Stub::EmptyFile("file.txt")]);

        nu!(
            cwd: dirs.test(),
            "touch -a file.txt"
        );

        let path = dirs.test().join("file.txt");

        // Check only the date since the time may not match exactly
        let date = Local::now().date_naive();
        let actual_date_time: DateTime<Local> =
            DateTime::from(path.metadata().unwrap().accessed().unwrap());
        let actual_date = actual_date_time.date_naive();

        assert_eq!(date, actual_date);
    })
}

#[test]
fn change_modified_and_access_time_of_file_to_today() {
    Playground::setup("change_time_test_27", |dirs, sandbox| {
        sandbox.with_files(vec![Stub::EmptyFile("file.txt")]);

        nu!(
            cwd: dirs.test(),
            "touch -a -m file.txt"
        );

        let metadata = dirs.test().join("file.txt").metadata().unwrap();

        // Check only the date since the time may not match exactly
        let date = Local::now().date_naive();
        let adate_time: DateTime<Local> = DateTime::from(metadata.accessed().unwrap());
        let adate = adate_time.date_naive();
        let mdate_time: DateTime<Local> = DateTime::from(metadata.modified().unwrap());
        let mdate = mdate_time.date_naive();

        assert_eq!(date, adate);
        assert_eq!(date, mdate);
    })
}

#[test]
fn not_create_file_if_it_not_exists() {
    Playground::setup("change_time_test_28", |dirs, _sandbox| {
        nu!(
            cwd: dirs.test(),
            "touch -c file.txt"
        );

        let path = dirs.test().join("file.txt");

        assert!(!path.exists());

        nu!(
            cwd: dirs.test(),
            "touch -c file.txt"
        );

        let path = dirs.test().join("file.txt");

        assert!(!path.exists());
    })
}

#[test]
fn creates_file_three_dots() {
    Playground::setup("create_test_1", |dirs, _sandbox| {
        nu!(
            cwd: dirs.test(),
            "touch file..."
        );

        let path = dirs.test().join("file...");
        assert!(path.exists());
    })
}

#[test]
fn creates_file_four_dots() {
    Playground::setup("create_test_1", |dirs, _sandbox| {
        nu!(
            cwd: dirs.test(),
            "touch file...."
        );

        let path = dirs.test().join("file....");
        assert!(path.exists());
    })
}

#[test]
fn creates_file_four_dots_quotation_marks() {
    Playground::setup("create_test_1", |dirs, _sandbox| {
        nu!(
            cwd: dirs.test(),
            "touch 'file....'"
        );

        let path = dirs.test().join("file....");
        assert!(path.exists());
    })
}
