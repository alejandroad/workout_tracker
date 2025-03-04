pub struct WorkoutEntry {
    pub exercise_name: String,
    pub weight: u8,
    pub sets: u8,
    pub reps: u8,
    pub perceived_effort: u8,
    pub date: String,
    pub description: String,
}

pub fn build_workout(
    exercise_name: &str,
    weight: u8,
    sets: u8,
    reps: u8,
    perceived_effort: u8,
    date: &str,
    description: &str,
) -> Result<WorkoutEntry, String> {
    if perceived_effort > 10 || perceived_effort < 1 { 
        return Err("Perceived effort must be an integer between 1 and 10".to_string());
    }

    Ok(WorkoutEntry {
        exercise_name: exercise_name.to_string(),
        weight,
        sets,
        reps,
        perceived_effort,
        date: date.to_string(),
        description: description.to_string(),
    })
}