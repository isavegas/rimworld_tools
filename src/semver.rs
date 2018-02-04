use std::cmp::Ordering;


#[derive(Debug, Eq)]
pub struct Version {
    major: i32,
    minor: i32,
    revision: i32,
}
#[allow(dead_code)] // This is an API. Of course we allow it.
impl Version {
    pub fn new(major: i32, minor: i32, revision: i32) -> Version {
        Version {
            major: major,
            minor: minor,
            revision: revision,
        }
    }
    pub fn from_str(in_str: &str) -> Result<Version, String> {
        if in_str.contains("+") || in_str.contains("-") {
            return Err(format!("Invalid semver {}", in_str));
        }
        let parts = in_str.split(".").map(|s| s.parse() ).collect::<Vec<Result<i32,_>>>();
        if parts.iter().any(|r| r.is_err()) {
            return Err(format!("Invalid semver {}", in_str));
        }
        let semver_parts = parts.into_iter().map(|r| r.unwrap()).collect::<Vec<i32>>();
        if semver_parts.len() != 3 {
            Err(format!("Invalid semver, wrong number of parts: {}", semver_parts.len()))
        } else {
            Ok(Version {
                major: semver_parts[0],
                minor: semver_parts[1],
                revision: semver_parts[2],
            })
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.revision)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        (&self.major, &self.minor, &self.revision).cmp(&(&other.major, &other.minor, &other.revision))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major
        && self.minor == other.minor
        && self.revision == other.revision
    }
}