
struct Post{
    title: String,
    author: String,
    likes: i32
}
fn main() {
    let mut posts = vec![
        Post{
            title: String::from("new game Xbox"),
            author: String::from("Woman"),
            likes: 3
        },
        Post{
            title: String::from("game Xbox 2"),
            author: String::from("Man"),
            likes: 2
        },
        Post{
            title: String::from("game Playstaition 5"),
            author: String::from("Japan"),
            likes: 6
        }
    ];
    posts.sort_by_key(|post| post.title.clone());
    display_posts(posts);

}

fn display_posts(posts: Vec<Post>){
    for post in posts{
        println!("{} - {} - {}", post.title, post.author, post.likes);
    }
}