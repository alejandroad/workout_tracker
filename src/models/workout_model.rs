pub struct WorkoutEntry {
    pub exercise_name: String,
    pub sets: u64,
    pub reps: u64,
    pub perceived_effort: u64,  // Fixed spelling to match table
    pub date: String,
    pub descriptions: String,
}
