use polars::prelude::*;

pub trait DataCleaner {
    fn clean_string(input: &str) -> String {
        input.trim().to_uppercase()
    }

    /// Normalizes business names (removes common suffixes like S.A.S, LTDA)
    fn normalize_business_name(name: &str) -> String {
        let clean = Self::clean_string(name);
        clean
            .replace(" S A S", "")
            .replace(" S.A.S.", "")
            .replace(" LTDA", "")
            .replace(" LIMITADA", "")
            .replace(".", "")
            .trim()
            .to_string()
    }
}

pub struct StandardCleaner;

impl DataCleaner for StandardCleaner {}
