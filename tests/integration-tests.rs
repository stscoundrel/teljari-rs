use teljari::join_with_conj;

#[test]
fn joins_with_conjunction() {
    let list = ["Revenger", "Shadow Captain", "Bone Silence"];
    let conj = "or possibly even";
    let expected = "Revenger, Shadow Captain or possibly even Bone Silence";
    let result = join_with_conj(&list, conj);
    
    assert_eq!(result, expected);
}