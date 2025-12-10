use validator::ValidationError;

/// Validate event title
pub fn validate_event_title(title: &str) -> Result<(), ValidationError> {
    if title.trim().is_empty() {
        return Err(ValidationError::new("event_title_required"));
    }
    
    if title.len() < 3 {
        return Err(ValidationError::new("event_title_too_short"));
    }
    
    if title.len() > 200 {
        return Err(ValidationError::new("event_title_too_long"));
    }
    
    Ok(())
}

/// Validate event description
pub fn validate_event_description(description: &str) -> Result<(), ValidationError> {
    if description.len() > 2000 {
        return Err(ValidationError::new("event_description_too_long"));
    }
    
    Ok(())
}

/// Validate event location
pub fn validate_event_location(location: &str) -> Result<(), ValidationError> {
    if location.len() > 500 {
        return Err(ValidationError::new("event_location_too_long"));
    }
    
    Ok(())
}

/// Validate event price
pub fn validate_event_price(price: f64) -> Result<(), ValidationError> {
    if price < 0.0 {
        return Err(ValidationError::new("event_price_negative"));
    }
    
    if price > 100000.0 {
        return Err(ValidationError::new("event_price_too_high"));
    }
    
    Ok(())
}

/// Validate max attendees
pub fn validate_max_attendees(max_attendees: u32) -> Result<(), ValidationError> {
    if max_attendees == 0 {
        return Err(ValidationError::new("max_attendees_zero"));
    }
    
    if max_attendees > 100000 {
        return Err(ValidationError::new("max_attendees_too_high"));
    }
    
    Ok(())
}

/// Validate event tags
pub fn validate_event_tags(tags: &[String]) -> Result<(), ValidationError> {
    if tags.len() > 20 {
        return Err(ValidationError::new("event_tags_too_many"));
    }
    
    for tag in tags {
        if tag.trim().is_empty() {
            return Err(ValidationError::new("event_tag_empty"));
        }
        
        if tag.len() > 50 {
            return Err(ValidationError::new("event_tag_too_long"));
        }
    }
    
    Ok(())
}

/// Validate organizer name
pub fn validate_organizer_name(name: &str) -> Result<(), ValidationError> {
    if name.trim().is_empty() {
        return Err(ValidationError::new("organizer_name_required"));
    }
    
    if name.len() < 2 {
        return Err(ValidationError::new("organizer_name_too_short"));
    }
    
    if name.len() > 100 {
        return Err(ValidationError::new("organizer_name_too_long"));
    }
    
    // Check if name contains only letters, spaces, and common punctuation
    if !name.chars().all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-' || c == '\'' || c == '.') {
        return Err(ValidationError::new("organizer_name_invalid_characters"));
    }
    
    Ok(())
}

/// Validate age restriction
pub fn validate_age_restriction(age_restriction: &str) -> Result<(), ValidationError> {
    if age_restriction.len() > 100 {
        return Err(ValidationError::new("age_restriction_too_long"));
    }
    
    Ok(())
}

/// Validate dress code
pub fn validate_dress_code(dress_code: &str) -> Result<(), ValidationError> {
    if dress_code.len() > 200 {
        return Err(ValidationError::new("dress_code_too_long"));
    }
    
    Ok(())
}

/// Validate special instructions
pub fn validate_special_instructions(instructions: &str) -> Result<(), ValidationError> {
    if instructions.len() > 1000 {
        return Err(ValidationError::new("special_instructions_too_long"));
    }
    
    Ok(())
}

/// Validate meeting link for virtual events
pub fn validate_meeting_link(link: &str) -> Result<(), ValidationError> {
    if link.trim().is_empty() {
        return Err(ValidationError::new("meeting_link_required"));
    }
    
    // Check if it's a valid URL
    if !link.starts_with("http://") && !link.starts_with("https://") {
        return Err(ValidationError::new("meeting_link_invalid_protocol"));
    }
    
    if link.len() > 500 {
        return Err(ValidationError::new("meeting_link_too_long"));
    }
    
    Ok(())
}

/// Validate registration URL
pub fn validate_registration_url(url: &str) -> Result<(), ValidationError> {
    if url.trim().is_empty() {
        return Err(ValidationError::new("registration_url_required"));
    }
    
    // Check if it's a valid URL
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ValidationError::new("registration_url_invalid_protocol"));
    }
    
    if url.len() > 500 {
        return Err(ValidationError::new("registration_url_too_long"));
    }
    
    Ok(())
}

/// Validate event date is in the future
pub fn validate_future_date(date: &chrono::DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    let now = chrono::Utc::now();
    
    if *date <= now {
        return Err(ValidationError::new("event_date_not_future"));
    }
    
    // Check if date is not too far in the future (e.g., 5 years)
    let max_future_date = now + chrono::Duration::days(365 * 5);
    if *date > max_future_date {
        return Err(ValidationError::new("event_date_too_far_future"));
    }
    
    Ok(())
}

/// Validate event end date is after start date
pub fn validate_event_dates(start_date: &chrono::DateTime<chrono::Utc>, end_date: &chrono::DateTime<chrono::Utc>) -> Result<(), ValidationError> {
    if end_date <= start_date {
        return Err(ValidationError::new("end_date_before_start_date"));
    }
    
    // Check if event duration is not too long (e.g., 30 days)
    let duration = *end_date - *start_date;
    if duration.num_days() > 30 {
        return Err(ValidationError::new("event_duration_too_long"));
    }
    
    Ok(())
}

/// Validate registration deadline is before event start
pub fn validate_registration_deadline(
    registration_deadline: &chrono::DateTime<chrono::Utc>,
    event_start: &chrono::DateTime<chrono::Utc>
) -> Result<(), ValidationError> {
    if registration_deadline >= event_start {
        return Err(ValidationError::new("registration_deadline_after_event_start"));
    }
    
    Ok(())
}

/// Validate event capacity (current attendees <= max attendees)
pub fn validate_event_capacity(current_attendees: u32, max_attendees: Option<u32>) -> Result<(), ValidationError> {
    if let Some(max) = max_attendees {
        if current_attendees > max {
            return Err(ValidationError::new("current_attendees_exceeds_max"));
        }
    }
    
    Ok(())
}

/// Validate phone number for international format
pub fn validate_international_phone(phone: &str) -> Result<(), ValidationError> {
    // Remove all non-digit characters except +
    let cleaned = phone.chars().filter(|c| c.is_ascii_digit() || *c == '+').collect::<String>();
    
    if cleaned.is_empty() {
        return Err(ValidationError::new("phone_number_required"));
    }
    
    // Check if it starts with + for international format
    if !cleaned.starts_with('+') {
        return Err(ValidationError::new("phone_number_international_format"));
    }
    
    // Check length (should be between 8 and 15 digits after country code)
    let digits_only = cleaned.chars().filter(|c| c.is_ascii_digit()).count();
    if digits_only < 8 || digits_only > 15 {
        return Err(ValidationError::new("phone_number_invalid_length"));
    }
    
    Ok(())
}

/// Validate company name
pub fn validate_company_name(company: &str) -> Result<(), ValidationError> {
    if company.trim().is_empty() {
        return Err(ValidationError::new("company_name_required"));
    }
    
    if company.len() < 2 {
        return Err(ValidationError::new("company_name_too_short"));
    }
    
    if company.len() > 200 {
        return Err(ValidationError::new("company_name_too_long"));
    }
    
    Ok(())
}

/// Validate designation/job title
pub fn validate_designation(designation: &str) -> Result<(), ValidationError> {
    if designation.trim().is_empty() {
        return Err(ValidationError::new("designation_required"));
    }
    
    if designation.len() < 2 {
        return Err(ValidationError::new("designation_too_short"));
    }
    
    if designation.len() > 100 {
        return Err(ValidationError::new("designation_too_long"));
    }
    
    Ok(())
}
