use crate::types::ApiError;

pub fn normalize_str(string: String) -> String {
    ic_cdk::println!("before alias--{:?}", &string);
    let after_string = string.trim().to_ascii_lowercase();
    ic_cdk::println!("after alias--{:?}", &after_string);
    after_string
}

pub fn validate_alias(alias: &str) -> Result<(), ApiError> {
    if !alias.chars().all(|char| char.is_ascii_alphanumeric()) {
        return Err(ApiError::AliasFormatFail(
            "Alias must be alphanumeric and ascii.".to_string(),
        ));
    }
    Ok(())
}


pub fn get_points_by_len(len: usize) -> u64 {
    match len {
        4 => 10000,
        5 => 5000,
        6 => 2000,
        7 => 800,
        8..=11 => 500,
        _ => 0

    }
}
