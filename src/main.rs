mod bst {

    #[derive(Debug)]
    pub struct Tree<T> {
        pub left: Option<Box<Tree<T>>>,
        pub right: Option<Box<Tree<T>>>,
        data: Option<Box<T>>,
    }

    impl<T> Tree<T>
        where
            T: PartialOrd + Copy,
    {
        pub fn new(val: T) -> Tree<T> {
            Tree {
                left: None,
                right: None,
                data: Some(Box::new(val)),
            }
        }

        pub fn data(&self) -> Box<Option<&Box<T>>> {
            Box::new(self.data.as_ref())
        }

        pub fn insert(&mut self, val: T) {
            if let Some(data) = self.data.as_ref() {
                if **data >= val {
                    if let Some(left) = self.left.as_mut() {
                        left.insert(val);
                    } else {
                        self.left = Some(Box::new(Tree::new(val)));
                    }
                } else {
                    if let Some(right) = self.right.as_mut() {
                        right.insert(val);
                    } else {
                        self.right = Some(Box::new(Tree::new(val)));
                    }
                }
            } else {
                self.data = Some(Box::new(val));
            }
        }

        pub fn get_in_order(&self,collection:&mut Vec<Box<T>>) -> () where T: Copy {
            self.in_order_collect(self,collection);
        }

        pub fn in_order_collect<'a>(&self, tree: &Tree<T>, collection: &'a mut Vec<Box<T>>) -> (){
            if let Some(__) = tree.left.as_ref() {
                self.in_order_collect(tree.left.as_ref().unwrap(), collection);
            }
            let v = tree.data().unwrap().clone();
            match &tree.data{
                Some(val) => {
                    collection.push( Box::new(*v));
                },
                None => {

                }
            }
            if let Some(_) = tree.right.as_ref() {
                self.in_order_collect(tree.right.as_ref().unwrap(), collection);
            }
        }

        pub fn get_pre_order(&self,collection:&mut Vec<Box<T>>) -> () {
            self.pre_order_collect(self,collection);
        }

        pub fn pre_order_collect<'a>(&self, tree: &Tree<T>, collection: &'a mut Vec<Box<T>>) -> () {
            let v = tree.data().unwrap().clone();
            match &tree.data{
                Some(val) => {
                    collection.push( Box::new(*v));
                },
                None => {

                }
            }
            if let Some(__) = tree.left.as_ref() {
                self.pre_order_collect(tree.left.as_ref().unwrap(), collection);
            }
            if let Some(_) = tree.right.as_ref() {
                self.pre_order_collect(tree.right.as_ref().unwrap(), collection);
            }
        }

        pub fn get_post_order(&self,collection:&mut Vec<Box<T>>) -> () {
            self.post_order_collect(self,collection);
        }

        pub fn post_order_collect<'a>(&self, tree: &Tree<T>, collection: &'a mut Vec<Box<T>>) -> () {
            let v = tree.data().unwrap().clone();
            if let Some(__) = tree.left.as_ref() {
                self.post_order_collect(tree.left.as_ref().unwrap(), collection);
            }
            if let Some(_) = tree.right.as_ref() {
                self.post_order_collect(tree.right.as_ref().unwrap(), collection);
            }
            match &tree.data{
                Some(val) => {
                    collection.push( Box::new(*v));
                },
                None => {

                }
            }
        }
    }
}

fn main() {
    let val: i64 = 99;
    let mut tree: bst::Tree<i64> = bst::Tree::new(val);
    tree.insert(77);

    println!("Tree: {:?}", tree);
}

#[cfg(test)]
mod test_bst {
    #[test]
    fn test_bst_create() {
        let mut tree = super::bst::Tree::new(77);
        assert_eq!(**tree.data().unwrap(), 77);
    }

    #[test]
    fn test_bst_insert_less_than() {
        let mut tree = super::bst::Tree::new(77);
        tree.insert(90);
        assert_eq!(**tree.data().unwrap(), 77);
        assert_eq!(**tree.right.as_ref().unwrap().data().as_ref().unwrap(), 90);
    }

    #[test]
    fn test_bst_greater_less_than() {
        let mut tree = super::bst::Tree::new(77);
        tree.insert(9);
        assert_eq!(**tree.data().unwrap(), 77);
        assert_eq!(**tree.left.as_ref().unwrap().data().as_ref().unwrap(), 9);
    }

    #[test]
    fn test_bst_get_in_order() {
        let mut tree: super::bst::Tree<i64> = super::bst::Tree::new(77);
        tree.insert(9);
        tree.insert(88);
        let mut list: Vec<Box<i64>> = vec!();
        tree.get_in_order(&mut list);
        assert_eq!(list, vec!(Box::new(9), Box::new(77), Box::new(88)));
    }

    #[test]
    fn test_bst_get_post_order() {
        let mut tree: super::bst::Tree<i64> = super::bst::Tree::new(77);
        tree.insert(9);
        tree.insert(88);
        let mut list: Vec<Box<i64>> = vec!();
        tree.get_post_order(&mut list);
        assert_eq!(list, vec!(Box::new(9), Box::new(88), Box::new(77)));
    }

    #[test]
    fn test_bst_get_pre_order() {
        let mut tree: super::bst::Tree<i64> = super::bst::Tree::new(77);
        tree.insert(9);
        tree.insert(88);
        let mut list: Vec<Box<i64>> = vec!();
        tree.get_pre_order(&mut list);
        assert_eq!(list, vec!(Box::new(77), Box::new(9), Box::new(88)));
    }
}
