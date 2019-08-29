pub mod linked_list;

#[cfg(test)]
mod test {
    use crate::linked_list::single_linked_list::*;
    use crate::linked_list::LinkedListNode;
    #[test]
    fn single_linked_list_test() {
        // Default test
        assert_eq!(1, 1);

        let mut list: SinglyLinkedList<usize> = SinglyLinkedList::new();

        list.tail_insert_node(SinglyLinkedListNode::create_node(Some(3), None));
        list.add_node(SinglyLinkedListNode::create_node(Some(2), None));
        list.head_insert_element(1);

        let vec: Vec<usize> = list.iter().collect();
        assert_eq!(vec, vec![1, 3, 2]);

        let mut list1: SinglyLinkedList<usize> = SinglyLinkedList::new();
        list1.head_insert_element(5);
        list1.add_node(list.get_head_node());

        let vec: Vec<usize> = list1.iter().collect();
        assert_eq!(vec, vec![5, 1, 3, 2]);

        let mut list2: SinglyLinkedList<usize> = SinglyLinkedList::new();
        list2.tail_insert_node(SinglyLinkedListNode::create_node(Some(7), None));
        list2.head_insert_element(6);
        list2.add_node(list.get_head_node());

        let vec: Vec<usize> = list2.iter().collect();
        assert_eq!(vec, vec![6, 7, 1, 3, 2]);
    }
}
