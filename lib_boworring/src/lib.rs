use lib_inventory::{Book, Inventory};
use lib_users::UserManager;

pub struct BorrowingService;


impl BorrowingService {


    pub fn new() -> Self {
        Self
    }

    pub fn borrow_book(&self, inventory: &mut Inventory, user_manager: &mut UserManager, user_id: u32, book_id: u32) -> Result<(), String> {

    // Check if the book exist 

    let book = inventory.get_book(book_id).ok_or_else(|| String::from("Book not found"))?;

    if !book.is_available {
        return Err("Book is not available".to_string())?;
    }

    let user = user_manager.get_user(user_id).ok_or_else(|| "User not found".to_string())?;

    if user.borrowed_books.len() >= 3 {
        return Err("User cannot borrow more than 3 books".to_string());
    }


    inventory.update_book_avalability(book_id, false)?;

    user_manager.borrow_book(user_id, book_id);

    Ok(())

    }


    pub fn return_book (&mut self,  inventory: &mut Inventory, user_manager: &mut UserManager, user_id: u32, book_id: u32) -> Result<(), String> {
     
     user_manager.return_book(user_id, book_id);

     inventory.update_book_avalability(book_id, true)?;

     Ok(())
    }
}



#[cfg(test)]
mod tests {
    use lib_inventory::{Genre, Book};
    use lib_users::User;

    use super::*;

    fn setup () -> (Inventory, UserManager, BorrowingService) {

        let mut inventory = Inventory::new();
        let mut user_manager = UserManager::new();
        let borrowing_service = BorrowingService::new();

        let book = Book {
            id: 1,
            title: String::from("Rust book"),
            author: String::from("Steve"),
            genre: Genre::Fiction,
            is_available: true,
        };

        inventory.add_book(book);

        // Add a user 

        let user = User {
            id: 1,
            name: String::from("Ayo"),
            borrowed_books: Vec::new(),
        };

        user_manager.register_user(user);

        (inventory, user_manager, borrowing_service)
    }

    #[test]
    fn test_borrow_and_return_book() {
        
        let (mut inventory, mut user_manager,mut  borrowing_service) = setup();

        assert!(borrowing_service.borrow_book(&mut inventory, &mut user_manager, 1, 1).is_ok());

        assert!(!inventory.get_book(1).unwrap().is_available);

        assert!(user_manager.get_user(1).unwrap().borrowed_books.contains(&1));

        assert!(borrowing_service.return_book(&mut inventory, &mut user_manager, 1, 1).is_ok());
    }
}
