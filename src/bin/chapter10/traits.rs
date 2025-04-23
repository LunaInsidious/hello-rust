use std::fmt::Display;

trait Summary {
  fn summary(&self) -> String {
    String::from("Read more...")
  }
}

struct NewsArticle {
  headline: String,
  location: String,
  author: String,
  content: String,
}

impl NewsArticle {
  fn new(
    headline: String,
    location: String,
    author: String,
    content: String,
  ) -> NewsArticle {
    NewsArticle {
      headline,
      location,
      author,
      content,
    }
  }
}

impl Summary for NewsArticle {}
impl Display for NewsArticle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "NewsArticle: {} by {} in {} \n{}",
      self.headline, self.author, self.location, self.content
    )
  }
}

struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summary(&self) -> String {
    format!(
      "{}: {}\nreply: {}, retweet: {}",
      self.username, self.content, self.reply, self.retweet
    )
  }
}
impl Display for Tweet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Tweet by {}: {}", self.username, self.content)
  }
}

fn notify(item: &(impl Summary + Display)) {
  println!("Breaking news! {}", item.summary());
  println!("Display: {}", item);
}
// 上記は下記の糖衣構文
// 引数2つに対して全く同じ型を強制する場合は、下記の書き方にする必要がある
fn notify2<T: Summary + Display>(item: &T) {
  println!("Breaking news! {}", item.summary());
  println!("Display: {}", item);
}
fn notify3<T, U>(item1: &T, item2: &U)
where
  T: Summary + Display,
  U: Summary + Display,
{
  println!("Breaking news1! {}", item1.summary());
  println!("Display: {}", item1);
  println!("Breaking news2! {}", item2.summary());
  println!("Display: {}", item2);
}

fn returns_summarizable() -> impl Summary {
  NewsArticle::new(
    String::from("Breaking News"),
    String::from("Tokyo"),
    String::from("John Doe"),
    String::from("This is the content of the news article."),
  )
}
// NewsArticleとTweetのどちらかを返す関数は書けない
// fn returns_summarizable2() -> impl Summary {
//   if true {
//     NewsArticle::new(
//       String::from("Breaking News"),
//       String::from("Tokyo"),
//       String::from("John Doe"),
//       String::from("This is the content of the news article."),
//     )
//   } else {
//     Tweet {
//       username: String::from("user123"),
//       content: String::from("This is a tweet."),
//       reply: false,
//       retweet: true,
//     }
//   }
// }

pub fn traits() {
  let article = NewsArticle::new(
    String::from("Breaking News"),
    String::from("Tokyo"),
    String::from("John Doe"),
    String::from("This is the content of the news article."),
  );

  let tweet = Tweet {
    username: String::from("user123"),
    content: String::from("This is a tweet."),
    reply: false,
    retweet: true,
  };

  println!("Article Summary: {}", article.summary());
  println!("Tweet Summary: {}", tweet.summary());

  notify(&article);
  notify(&tweet);
  notify2(&article);
  notify2(&tweet);
  notify3(&article, &tweet);

  let article2 = returns_summarizable();
  println!("Returned Article Summary: {}", article2.summary());
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

pub fn largest_trait() {
  let numbers = vec![34, 50, 25, 100, 65];
  let result = largest(&numbers);
  println!("The largest number is: {}", result);
  let chars = vec!['y', 'm', 'a', 'q'];
  let result = largest(&chars);
  println!("The largest char is: {}", result);
}

struct Pair<T> {
  x: T,
  y: T,
}

impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Pair { x, y }
  }
}

impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x > self.y {
      println!("The largest member is x: {}", self.x);
    } else {
      println!("The largest member is y: {}", self.y);
    }
  }
}

pub fn impl_trait() {
  let pair = Pair::new(1, 2);
  pair.cmp_display();
  let pair2 = Pair::new(1.0, 2.0);
  pair2.cmp_display();
  let pair3 = Pair::new("hello", "world");
  pair3.cmp_display();
  //   let pair4 = Pair::new([1, 2, 3], [4, 5, 6]);
  //   pair4.cmp_display();
}
