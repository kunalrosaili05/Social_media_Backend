use std::collections::HashMap;
use std::fs::{self};
use std::io::{self, Write};
use serde::{Deserialize, Serialize};

// Represents a social media post with attributes for its ID, content, comments, likes, and dislikes.
#[derive(Debug, Serialize, Deserialize)]
struct Post {                       //Structs are used to create custom data types.
    id: usize,                      // Unique identifier for the post
    content: String,                // The content of the post
    comments: Vec<String>,          // A list of comments made on the post
    likes: usize,                   // Number of likes the post has received
    dislikes: usize,                // Number of dislikes the post has received
}

impl Post {
    // Constructs a new Post instance with a given ID and content.
    fn new(id: usize, content: String) -> Post {
        Post {
            id,
            content,
            comments: Vec::new(),    // Initializes comments as an empty vector
            likes: 0,                // Sets initial likes to 0
            dislikes: 0,             // Sets initial dislikes to 0
        }
    }

    // Adds a new comment to the post.
    fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);  // Pushes the new comment onto the comments vector
    }

    // Increments the like count for the post.
    fn like(&mut self) {
        self.likes += 1;              // Increases the like count by 1
    }

    // Increments the dislike count for the post.
    fn dislike(&mut self) {
        self.dislikes += 1;           // Increases the dislike count by 1
    }

    // Generates a shareable link for the post based on its ID.
    fn share_link(&self) -> String {
        format!("http://myapp.com/post/{}", self.id)  // Formats the URL with the post ID
    }
}

// Manages a collection of posts, providing functionalities to manipulate them.
#[derive(Serialize, Deserialize)]
struct PostManager {
    posts: HashMap<usize, Post>,    // Stores posts with their unique IDs as keys
    next_id: usize,                  // Keeps track of the next available post ID
}

impl PostManager {
    // Creates a new PostManager instance, loading existing posts from a file if available.
    fn new() -> PostManager {
        // Try loading data from the saved file, if it exists
        if let Ok(data) = fs::read_to_string("posts.json") {
            if let Ok(post_manager) = serde_json::from_str(&data) {
                return post_manager;  // Returns the loaded post manager
            }
        }
        // If loading fails, create a new instance
        PostManager {
            posts: HashMap::new(),    // Initializes with an empty HashMap
            next_id: 1,                // Starts ID counting from 1
        }
    }

    // Saves the current posts to a JSON file for persistence.
    fn save_data(&self) {
        // Save the data to a JSON file
        if let Ok(json_data) = serde_json::to_string(&self) {
            let _ = fs::write("posts.json", json_data);  // Writes JSON data to the file
        }
    }

    // Creates a new post and adds it to the collection, returning its ID.
    fn create_post(&mut self, content: String) -> usize {
        let post = Post::new(self.next_id, content);  // Creates a new Post
        self.posts.insert(self.next_id, post);         // Inserts the new post into the HashMap
        self.next_id += 1;                             // Increments the next ID for future posts
        self.save_data();                              // Save after creating a post
        self.next_id - 1                               // Returns the ID of the newly created post
    }

    // Adds a comment to an existing post by its ID.
    fn add_comment(&mut self, post_id: usize, comment: String) {
        if let Some(post) = self.posts.get_mut(&post_id) {  // Checks if the post exists
            post.add_comment(comment);                       // Adds the comment to the post
            self.save_data();                                // Save after adding a comment
        } else {
            println!("Post with ID {} does not exist.", post_id); // Error message if the post is not found
        }
    }

    // Likes a post identified by its ID.
    fn like_post(&mut self, post_id: usize) {
        if let Some(post) = self.posts.get_mut(&post_id) {  // Checks if the post exists
            post.like();                                    // Increments the like count
            self.save_data();                               // Save after liking
        } else {
            println!("Post with ID {} does not exist.", post_id); // Error message if the post is not found
        }
    }

    // Dislikes a post identified by its ID.
    fn dislike_post(&mut self, post_id: usize) {
        if let Some(post) = self.posts.get_mut(&post_id) {  // Checks if the post exists
            post.dislike();                                   // Increments the dislike count
            self.save_data();                                 // Save after disliking
        } else {
            println!("Post with ID {} does not exist.", post_id); // Error message if the post is not found
        }
    }

    // Deletes a post identified by its ID.
    fn delete_post(&mut self, post_id: usize) {
        if self.posts.remove(&post_id).is_some() {         // Attempts to remove the post
            println!("Post with ID {} has been deleted.", post_id); // Confirmation message
            self.save_data();                               // Save after deleting
        } else {
            println!("Post with ID {} does not exist.", post_id); // Error message if the post is not found
        }
    }

    // Shares a post by displaying its shareable link.
    fn share_post(&self, post_id: usize) {
        if let Some(post) = self.posts.get(&post_id) {    // Checks if the post exists
            println!("Sharing post: {}", post.share_link()); // Displays the shareable link
        } else {
            println!("Post with ID {} does not exist.", post_id); // Error message if the post is not found
        }
    }

    // Displays all the posts currently managed.
    fn display_posts(&self) {
        for post in self.posts.values() {                  // Iterates over all posts
            println!("{:?}", post);                        // Prints each post's debug representation
        }
    }
}

// The entry point of the application.
fn main() {
    let mut post_manager = PostManager::new();          // Initializes the PostManager

    loop {
        // Display menu options to the user
        println!("\nSelect an action:");
        println!("1. Create a Post");
        println!("2. Add Comment to a Post");
        println!("3. Like a Post");
        println!("4. Dislike a Post");
        println!("5. Share a Post");
        println!("6. Display All Posts");
        println!("7. Delete a Post");
        println!("8. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap(); // Reads the user's choice from input
        let choice: u32 = choice.trim().parse().unwrap_or(0); // Parses the input to an integer

        match choice {
            1 => {
                print!("Enter post content: ");
                io::stdout().flush().unwrap(); // Prompt for post content
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap(); // Reads the post content
                let post_id = post_manager.create_post(content.trim().to_string()); // Creates a post
                println!("Post created with ID: {}", post_id); // Confirms creation
            }
            2 => {
                print!("Enter post ID to comment on: ");
                io::stdout().flush().unwrap(); // Prompt for post ID
                let mut post_id = String::new();
                io::stdin().read_line(&mut post_id).unwrap(); // Reads the post ID
                let post_id: usize = post_id.trim().parse().unwrap_or(0); // Parses post ID

                print!("Enter your comment: ");
                io::stdout().flush().unwrap(); // Prompt for comment
                let mut comment = String::new();
                io::stdin().read_line(&mut comment).unwrap(); // Reads the comment
                post_manager.add_comment(post_id, comment.trim().to_string()); // Adds the comment
            }
            3 => {
                print!("Enter post ID to like: ");
                io::stdout().flush().unwrap(); // Prompt for post ID
                let mut post_id = String::new();
                io::stdin().read_line(&mut post_id).unwrap(); // Reads the post ID
                let post_id: usize = post_id.trim().parse().unwrap_or(0); // Parses post ID
                post_manager.like_post(post_id); // Likes the post
            }
            4 => {
                print!("Enter post ID to dislike: ");
                io::stdout().flush().unwrap(); // Prompt for post ID
                let mut post_id = String::new();
                io::stdin().read_line(&mut post_id).unwrap(); // Reads the post ID
                let post_id: usize = post_id.trim().parse().unwrap_or(0); // Parses post ID
                post_manager.dislike_post(post_id); // Dislikes the post
            }
            5 => {
                print!("Enter post ID to share: ");
                io::stdout().flush().unwrap(); // Prompt for post ID
                let mut post_id = String::new();
                io::stdin().read_line(&mut post_id).unwrap(); // Reads the post ID
                let post_id: usize = post_id.trim().parse().unwrap_or(0); // Parses post ID
                post_manager.share_post(post_id); // Shares the post
            }
            6 => {
                post_manager.display_posts(); // Displays all posts
            }
            7 => {
                print!("Enter post ID to delete: ");
                io::stdout().flush().unwrap(); // Prompt for post ID
                let mut post_id = String::new();
                io::stdin().read_line(&mut post_id).unwrap(); // Reads the post ID
                let post_id: usize = post_id.trim().parse().unwrap_or(0); // Parses post ID
                post_manager.delete_post(post_id); // Deletes the post
            }
            8 => {
                break; // Exits the loop to terminate the program
            }
            _ => {
                println!("Invalid choice. Please try again."); // Error message for invalid input
            }
        }
    }
}
