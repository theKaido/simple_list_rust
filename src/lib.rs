use std::iter::FromIterator;

#[derive(Debug)]

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    head : Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList{head : None}
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut leng = 0;
        let mut p = &self.head;// p ici parcour la liste
        while let Some(a) = p {//si p a une valeur dans ces cellule 
            p = &a.next; // on parcour la liste tant que la boucle while est vrai 
            leng += 1;// on ajoute 1 au compteur leng
        }//un fois quil n'y a plus rien on retourne leng
        return leng;// on return leng a la fin du parcours
        
    }

    pub fn push(&mut self, element: T) {
        let old_node = self.head.take();// on récupere la valeur head avec le .take()
        //.take() Takes the value out of the option, leaving a None in its place.
        let new_node = Box::new(Node{  // on créer une nouvelle cellule 
            data : element, // ici on ajoute la valeur
            next : old_node,// ici on l'ajoute a notre linked list
        });
        self.head = Some(new_node) // on retourne le parametre
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut node) = self.head.take() { //on verifie si node à une valeur dedans si oui on retire la valeur avec take()
            self.head = node.next.take(); //on selectionne avec take le pointeur suivant que l'on va retiré
            Some(node.data) // on regarde si il y'a une valeur grace à la fonction some
        } else {
            None // sinon on renvoie rien 
        }
    }


    pub fn peek(&self) -> Option<&T> {
       match &self.head {//on créer un pattern matching et onverifie les condition suivantes
           None => None,//si il n'ya rien on return rien 
           Some(a) => { //a appartient à la struct SimpleListe 
               Some(&a.data)//si il ya quelchose on renvoie cette valeur
           }

       }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
    let mut new_list = self; // Ici on créer tout simplement un Liste mutable qui va prendre en paramètre
    let mut rev_list= Self::new();// on créer le conteneur des valeur inversé
    while let Some(data) = new_list.pop(){//tant qu'il ya quelque chose dans data on va affiché sa valeur
        rev_list.push(data);
        }
    return rev_list;
    }
}

impl<T> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list_to_iter = SimpleLinkedList::new();
        for i in _iter  {
            list_to_iter.push(i);
            
        }
        return list_to_iter;
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        unimplemented!()
    }
}
