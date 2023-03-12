use postgres::{Client, Error, NoTls};
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Error> {
    // Создаем клиент для подключения к базе данных Postgres
    let mut client  = Client::connect(
        "postgresql://your_username:your_password@localhost/your_database",
        NoTls,
    )?;

    // Создаем таблицу books (если ее нет)
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS books (
                id SERIAL PRIMARY KEY,
                title TEXT NOT NULL,
                author TEXT NOT NULL
            )",
            &[],
        )?;

    // Создаем книгу в таблице
    let title = "The Great Gatsby";
    let author = "F. Scott Fitzgerald";
    client
        .execute(
            "INSERT INTO books (title, author) VALUES ($1, $2)",
            &[&title, &author],
        )?;
    println!("Book created successfully!");

    // Читаем книгу из таблицы
    let rows = client
        .query("SELECT * FROM books WHERE title = $1", &[&title])?;
    let row = &rows[0];
    let id: i32 = row.get(0);
    let title: String = row.get(1);
    let author: String = row.get(2);
    println!("Book found: id={}, title='{}', author='{}'", id, title, author);

    // Обновляем книгу в таблице
    let new_title = "The Great Gatsby (Revised Edition)";
    client
        .execute(
            "UPDATE books SET title = $1 WHERE id = $2",
            &[&new_title, &id],
        )?;
    println!("Book updated successfully!");

    // Удаляем книгу из таблицы
    // client.execute("DELETE FROM books WHERE id = $1", &[&id])?;
    // println!("Book deleted successfully!");

    Ok(())
}
