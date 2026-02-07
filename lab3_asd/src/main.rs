use std::cmp::Ordering;

// --- Structura Nod ---
#[derive(Debug)]
struct Node {
    value: i32,
    left: Link,
    right: Link,
}

// Definim tipul alias Link pentru a simplifica codul.
// Box<Node> alocă nodul pe heap. Option<> gestionează cazul None (null).
type Link = Option<Box<Node>>;

impl Node {
    fn new(value: i32) -> Box<Self> {
        Box::new(Node {
            value,
            left: None,
            right: None,
        })
    }
}

// --- Structura Binary Search Tree (BST) ---
#[derive(Debug)]
pub struct BinarySearchTree {
    root: Link,
}

impl BinarySearchTree {
    /// Creează un BST nou și gol.
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // --- Operația INSERT (Inserați) O(h) ---

    /// Inserează o valoare nouă în BST. Returnează true dacă inserarea a reușit.
    pub fn insert(&mut self, value: i32) -> bool {
        // Trecem referința mutabilă la root-ul arborelui
        Self::insert_recursive(&mut self.root, value)
    }

    // Funcție recursivă pentru inserare. Primeste o referință mutabilă la Link.
    fn insert_recursive(link: &mut Link, value: i32) -> bool {
        match link {
            Some(node) => {
                // Nodul curent există
                match value.cmp(&node.value) {
                    Ordering::Less => {
                        // Continuăm pe stânga
                        Self::insert_recursive(&mut node.left, value)
                    }
                    Ordering::Greater => {
                        // Continuăm pe dreapta
                        Self::insert_recursive(&mut node.right, value)
                    }
                    Ordering::Equal => {
                        // Valoarea deja există (nu se acceptă duplicate)
                        false
                    }
                }
            }
            None => {
                // Locul găsit: inserăm nodul nou
                *link = Some(Node::new(value));
                true
            }
        }
    }

    // --- Operația DELETE (Ștergere) O(h) ---

    /// Șterge nodul cu valoarea dată. Returnează true dacă ștergerea a reușit.
    pub fn delete(&mut self, value: i32) -> bool {
        Self::delete_recursive(&mut self.root, value)
    }

    // Funcție recursivă pentru ștergere
    fn delete_recursive(link: &mut Link, value: i32) -> bool {
        // Mutăm Link-ul curent, astfel încât să putem manipula Link-ul &mut node.left/right
        let mut node = match link.take() {
            Some(node) => node,
            None => return false, // Nodul nu există
        };

        match value.cmp(&node.value) {
            Ordering::Less => {
                // Căutăm în stânga
                let deleted = Self::delete_recursive(&mut node.left, value);
                *link = Some(node); // Restaurăm Link-ul
                deleted
            }
            Ordering::Greater => {
                // Căutăm în dreapta
                let deleted = Self::delete_recursive(&mut node.right, value);
                *link = Some(node); // Restaurăm Link-ul
                deleted
            }
            Ordering::Equal => {
                // Nodul de șters a fost găsit!

                // Cazul 1: Nod frunză sau cu un singur copil (stânga)
                if node.right.is_none() {
                    // Când *link devine node.left, vechiul 'node' este dezalocat automat
                    *link = node.left.take();
                }
                // Cazul 2: Nod cu un singur copil (dreapta)
                else if node.left.is_none() {
                    *link = node.right.take();
                }
                // Cazul 3: Nod cu doi copii
                else {
                    // Găsim succesorul (cel mai mic din subarborele drept)
                    let successor = Self::find_min_recursive(&mut node.right);

                    // Schimbăm valoarea nodului curent cu valoarea succesorului
                    node.value = successor.value;

                    *link = Some(node); // Restaurăm Link-ul
                }
                true
            }
        }
    }

    // Funcție helper: Extrage nodul cu cea mai mică valoare dintr-un subarbore.
    // Nodul minim este *dezalocat* din subarborele său original.
    fn find_min_recursive(link: &mut Link) -> Box<Node> {
        match link.as_mut() {
            Some(node) => {
                if node.left.is_none() {
                    // Nodul minim găsit, îl extragem ('take') și îl returnăm
                    // Acest `take()` îl șterge efectiv din arbore
                    link.take().unwrap()
                } else {
                    Self::find_min_recursive(&mut node.left)
                }
            }
            // Nu ar trebui să se întâmple dacă este apelat corect din delete
            None => panic!("Called find_min on empty tree/subtree!"),
        }
    }

    // --- Operația MODIFY (Modificare) O(h) ---

    /// Modifică valoarea unui nod existent.
    /// Eșuează dacă valoarea veche nu există sau valoarea nouă deja există.
    pub fn modify(&mut self, old_value: i32, new_value: i32) -> bool {
        // Căutare pentru a verifica dacă noua valoare există (O(h))
        if Self::search_recursive(&self.root, new_value) {
            println!(
                "Eroare: Valoarea nouă ({}) deja există în arbore.",
                new_value
            );
            return false;
        }

        // 1. Șterge vechea valoare (O(h))
        if self.delete(old_value) {
            // 2. Inserează noua valoare (O(h))
            self.insert(new_value);
            true
        } else {
            // Nodul cu old_value nu a fost găsit
            println!(
                "Eroare: Valoarea veche ({}) nu a fost găsită pentru modificare.",
                old_value
            );
            false
        }
    }

    // --- Funcție de Căutare (Helper) O(h) ---

    /// Caută o valoare în BST.
    pub fn sear ch(&self, value: i32) -> bool {
        Self::search_recursive(&self.root, value)
    }

    fn search_recursive(link: &Link, value: i32) -> bool {
        match link {
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => Self::search_recursive(&node.left, value),
                Ordering::Greater => Self::search_recursive(&node.right, value),
                Ordering::Equal => true,
            },
            None => false,
        }
    }

    // --- Funcție de Afișare (Helper) ---

    /// O metodă simplă de afișare In-Order (pentru verificare)
    pub fn print_in_order(&self) {
        print!("Arbore (In-Order): ");
        Self::in_order_traverse(&self.root);
        println!();
    }

    fn in_order_traverse(link: &Link) {
        if let Some(node) = link {
            Self::in_order_traverse(&node.left);
            print!("{} ", node.value);
            Self::in_order_traverse(&node.right);
        }
    }
}

// --- Exemplu de Utilizare ---

pub fn main() {
    println!("### Testare Binary Search Tree (BST) - Fără Rc/RefCell ###");
    let mut bst = BinarySearchTree::new();

    // --- INSERT ---
    println!("\n## INSERT (Inserați) ##");
    bst.insert(50);
    bst.insert(30);
    bst.insert(70);
    bst.insert(20);
    bst.insert(40);
    bst.insert(60);
    bst.insert(80);

    bst.print_in_order();

    // --- DELETE ---
    println!("\n## DELETE (Ștergere) ##");

    // Șterge nod cu doi copii (Root)
    println!("Șterge 50 (Doi copii): {}", bst.delete(50)); // true
    bst.print_in_order(); // Succesorul (60) a luat locul lui 50

    // Șterge un nod inexistent
    println!("Șterge 99 (Inexistent): {}", bst.delete(99)); // false

    // Șterge nod frunză
    println!("Șterge 80 (Frunză): {}", bst.delete(80)); // true
    bst.print_in_order();

    // --- MODIFY ---
    println!("\n## MODIFY (Modificare) ##");

    // Modificare reușită (70 -> 75)
    println!("Modifică 70 la 75: {}", bst.modify(70, 75)); // true
    bst.print_in_order();

    // Modificare eșuată (valoarea nouă există)
    println!("Modifică 60 la 75: {}", bst.modify(60, 75)); // false
    bst.print_in_order();
}
