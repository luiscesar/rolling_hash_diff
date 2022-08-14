use super::now_as_millis;


#[test]
fn test_rdiff_util_now_as_millis_case1() {
    let now = now_as_millis();
    println!("now {}", now);
    let now = now_as_millis();
    println!("now {}", now);
}