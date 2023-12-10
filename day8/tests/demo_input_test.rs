include!("./../src/main.rs");

#[test]
fn test_task_1() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input_demo.txt")?;
    let mut lines = input.lines();
    let directions = crate::parse_directions(lines.next().unwrap());
    let mut dir_map: HashMap<String, (String, String)> = HashMap::new();
    lines.next();

    for line in lines {
        let (key, value) = crate::parse_line(line);
        dir_map.insert(key, value);
    }

    let task_1 = crate::follow_directions(directions, dir_map);
    assert_eq!(task_1, 6);
    Ok(())
}


#[test]
fn test_task_2() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input_demo_2.txt")?;
    let mut lines = input.lines();
    let directions = crate::parse_directions(lines.next().unwrap());
    let mut dir_map: HashMap<String, (String, String)> = HashMap::new();
    lines.next();

    for line in lines {
        let (key, value) = crate::parse_line(line);
        dir_map.insert(key, value);
    }

    let task_2 = crate::follow_directions_2(directions, dir_map);
    assert_eq!(task_2, 6);
    Ok(())
}