use crate::cleaner::DataCleaner;
use crate::domain::Contractor;
use std::collections::HashMap;
use strsim::jaro_winkler;
use uuid::Uuid;

/// Threshold for fuzzy matching (0.0 - 1.0)
const FUZZY_THRESHOLD: f64 = 0.85;

/// Result of an entity match attempt
#[derive(Debug, Clone)]
pub enum MatchResult {
    /// Exact match found (deterministic)
    ExactMatch(Uuid),
    /// Fuzzy match found with confidence score
    FuzzyMatch { id: Uuid, confidence: f64 },
    /// No match found - new entity
    NoMatch,
}

/// Entity Resolution Engine for MDM
pub struct EntityResolver {
    /// Index: legal_id -> Contractor UUID
    id_index: HashMap<String, Uuid>,
    /// Index: normalized_name -> Contractor UUID
    name_index: HashMap<String, Uuid>,
    /// All known contractors
    contractors: HashMap<Uuid, Contractor>,
}

impl EntityResolver {
    pub fn new() -> Self {
        Self {
            id_index: HashMap::new(),
            name_index: HashMap::new(),
            contractors: HashMap::new(),
        }
    }

    /// Register a new contractor in the resolver
    pub fn register(&mut self, contractor: Contractor) {
        let id = contractor.id;

        // Index by legal ID (NIT/Cédula)
        self.id_index.insert(contractor.legal_id.clone(), id);

        // Index by normalized name
        let normalized = crate::cleaner::StandardCleaner::normalize_business_name(&contractor.name);
        self.name_index.insert(normalized, id);

        self.contractors.insert(id, contractor);
    }

    /// Resolve an entity: find existing or return NoMatch
    pub fn resolve(&self, legal_id: &str, name: &str) -> MatchResult {
        // Step 1: Deterministic match by legal ID
        let clean_id = legal_id.trim().replace("-", "").replace(".", "");
        if let Some(&id) = self.id_index.get(&clean_id) {
            return MatchResult::ExactMatch(id);
        }

        // Step 2: Fuzzy match by name
        let normalized_name = crate::cleaner::StandardCleaner::normalize_business_name(name);

        // Check exact name match first
        if let Some(&id) = self.name_index.get(&normalized_name) {
            return MatchResult::ExactMatch(id);
        }

        // Fuzzy search through all names
        let mut best_match: Option<(Uuid, f64)> = None;

        for (existing_name, &id) in &self.name_index {
            let similarity = jaro_winkler(&normalized_name, existing_name);

            if similarity >= FUZZY_THRESHOLD {
                if best_match.is_none() || similarity > best_match.unwrap().1 {
                    best_match = Some((id, similarity));
                }
            }
        }

        match best_match {
            Some((id, confidence)) => MatchResult::FuzzyMatch { id, confidence },
            None => MatchResult::NoMatch,
        }
    }

    /// Get a contractor by UUID
    pub fn get(&self, id: &Uuid) -> Option<&Contractor> {
        self.contractors.get(id)
    }

    /// Get mutable reference to update a contractor
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut Contractor> {
        self.contractors.get_mut(id)
    }

    /// Total number of registered contractors
    pub fn len(&self) -> usize {
        self.contractors.len()
    }

    /// Check if resolver is empty
    pub fn is_empty(&self) -> bool {
        self.contractors.is_empty()
    }
}

impl Default for EntityResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match_by_nit() {
        let mut resolver = EntityResolver::new();

        let contractor = Contractor::new(
            "Empresa ABC S.A.S.".to_string(),
            "900123456".to_string(),
        );
        let expected_id = contractor.id;
        resolver.register(contractor);

        let result = resolver.resolve("900123456", "Empresa ABC");
        assert!(matches!(result, MatchResult::ExactMatch(id) if id == expected_id));
    }

    #[test]
    fn test_fuzzy_match_by_name() {
        let mut resolver = EntityResolver::new();

        let contractor = Contractor::new(
            "CONSTRUCTORA NACIONAL S.A.S.".to_string(),
            "900111222".to_string(),
        );
        let expected_id = contractor.id;
        resolver.register(contractor);

        // Different NIT, slightly different name (typo/variation)
        let result = resolver.resolve("999999999", "CONSTRUCTORA NACONAL"); // Missing 'I'
        assert!(matches!(result, MatchResult::FuzzyMatch { id, confidence }
            if id == expected_id && confidence > 0.85));
    }

    #[test]
    fn test_no_match() {
        let mut resolver = EntityResolver::new();

        let contractor = Contractor::new(
            "Empresa XYZ".to_string(),
            "900999888".to_string(),
        );
        resolver.register(contractor);

        let result = resolver.resolve("111222333", "Totally Different Company");
        assert!(matches!(result, MatchResult::NoMatch));
    }
}
