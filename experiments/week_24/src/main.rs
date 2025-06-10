// Exercice: Navigation Data Extractor
// Objectif: Comprendre quand cloner vs référencer

struct NavigationData<'a> {
    waypoints: Vec<&'a str>,
    current_heading: String,
}

impl<'a> NavigationData<'a> {
    fn new(waypoints: Vec<&'a str>, heading: String) -> Self {
        Self {
            waypoints,
            current_heading: heading,
        }
    }

    // TODO: Implémenter cette fonction
    // Retourne: (heading owned, premiers 3 waypoints en &str)
    fn get_course_summary(&self) -> (String, Vec<&'a str>) {
        let heading = &self.current_heading;
        let vec_str = &self.waypoints;

        (heading.to_string(), vec_str.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_summary() {
        let nav = NavigationData::new(
            vec!["Port", "Starboard", "North", "East"],
            "270°".to_string(),
        );

        let (heading, waypoints) = nav.get_course_summary();

        assert_eq!(heading, "270°");
        assert_eq!(waypoints, vec!["Port", "Starboard", "North", "East"]);
        assert_eq!(waypoints.len(), 4);
    }

    #[test]
    fn test_references_valid() {
        let nav = NavigationData::new(vec!["Alpha", "Beta"], "180°".to_string());

        let (_heading, waypoints) = nav.get_course_summary();
        // Les références doivent rester valides tant que nav existe
        assert_eq!(waypoints[0], "Alpha");
    }
}
