use DataStruture::linked_list::*;

fn single_linked_list_test(){
    // Default test
    assert_eq!(1, 1);

    let mut list : SinglyLinkedList<usize> = SinglyLinkedList::new();
    let mut list1 : SinglyLinkedList<usize> = SinglyLinkedList::new();


    list.tail_insert_node(SinglyLinkedListNode::create_node(Some(3)));
    list.add_node(SinglyLinkedListNode::create_node(Some(2)));
    list.head_insert_element(1);

    let vec : Vec<usize> = list.iter().collect();
    assert_eq!(vec, vec![1,3,2]);

    list1.head_insert_element(5);
    list1.add_node(list.get_head_node());

    let vec : Vec<usize> = list1.iter().collect();
    assert_eq!(vec, vec![5,1,3,2]);
}


fn main(){
    single_linked_list_test();
}
