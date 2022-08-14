use std::time::SystemTime;

pub(crate) fn now_as_millis() -> u128 {
    let now = 
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    now
}


#[cfg(test)]
mod tests;
