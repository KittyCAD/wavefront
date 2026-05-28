use wavefront::Obj;

const TEST_CONTENT: &str = r#"
v 0.0 1.0 0.0 1.0 0.0 0.0
v -1.0 0.0 0.0 0.0 1.0 0.0
v 1.0 0.0 0.0 0.0 0.0 1.0
o Triangle
f 1 2 3
"#;

#[test]
fn color() {
    let obj = Obj::from_lines(TEST_CONTENT.lines()).unwrap();
    let expected_positions = [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
    let expected_colors = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    assert_eq!(obj.positions(), &expected_positions);
    assert_eq!(obj.colors(), &expected_colors);
    let triangle = obj.object("Triangle").unwrap();
    for vertices in triangle.triangles() {
        for (vi, vertex) in vertices.iter().enumerate() {
            assert_eq!(vertex.position(), expected_positions[vi]);
            assert_eq!(vertex.color(), Some(expected_colors[vi]));
        }
    }

    println!("{}", obj);
}
