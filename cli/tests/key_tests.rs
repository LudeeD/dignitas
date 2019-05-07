use clignitas;

#[test]
fn test_key_life() {
    // test that if I write a key to a file and then
    // read it from that file the same key is read
    // Testing if I don't screw up the writing/reading
    let key_written = clignitas::generate_key();

    // Write To File
    let test_file = String::from("test.key");
    clignitas::key_to_file(key_written.as_ref(), test_file.clone());

    // Read From File
    let key_read = clignitas::key_from_file(&test_file);

    assert_eq!(key_written.as_hex(), key_read.as_hex());
}
