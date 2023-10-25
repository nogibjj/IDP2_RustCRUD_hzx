#[cfg(test)]
mod tests {
    use etl_rust::{extract, transform_load, query, create_general_item, update_general_item, delete_general_item};

    use super::*;
    use std::fs;
    use std::path::Path;

    const TEST_DATABASE_FILE: &str = "GroceryDB.sqlite";
    const TEST_CSV_FILE: &str = "data/GroceryDB_IgFPro.csv";
    const TEST_URL: &str = "https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv";

    #[test]
    pub fn test_extract() {
        // Ensure test file does not exist before extraction
        if Path::new(TEST_CSV_FILE).exists() {
            fs::remove_file(TEST_CSV_FILE).expect("Failed to remove test CSV file");
        }

        assert!(extract(TEST_URL, TEST_CSV_FILE).is_ok());
        assert!(Path::new(TEST_CSV_FILE).exists());
    }

    #[test]
    pub fn test_transform_load() {
        // Ensure test database file does not exist before transformation and loading
        if Path::new(TEST_DATABASE_FILE).exists() {
            fs::remove_file(TEST_DATABASE_FILE).expect("Failed to remove test database file");
        }

        let result = transform_load(TEST_CSV_FILE);
        assert!(result.is_ok(), "test_transform_load failed: {:?}", result);
        assert!(Path::new(TEST_DATABASE_FILE).exists());
    }

    #[test]
    pub fn test_query() {
        // Assume transform_load() has been tested and CSV data is loaded into the database
        assert!(query().is_ok());
    }

    #[test]
    pub fn test_create_general_item() {
        assert!(create_general_item(
            "TestItem",
            10,
            0.5,
            0.25,
            2.5,
            15.5,
            "TestName",
            "TestNode"
        )
        .is_ok());

    }

    #[test]
    pub fn test_update_general_item() {
        // Assume create_general_item() has been tested and an item with specific ID exists

        let result = update_general_item(
            1,
            "UpdatedItem",
            20,
            0.75,
            0.35,
            2.75,
            15.75,
            "UpdatedName",
            "UpdatedNode",
        );

        assert!(result.is_ok(), "update_general_item failed: {:?}", result);
    }

    #[test]
    pub fn test_delete_general_item() {
        // Assume create_general_item() has been tested and an item with specific ID exists
        assert!(delete_general_item(1).is_ok());
    }
}

fn main(){
    tests::test_extract();
    tests::test_transform_load();
    tests::test_query();
    tests::test_create_general_item();
    tests::test_update_general_item();
    tests::test_delete_general_item();
}