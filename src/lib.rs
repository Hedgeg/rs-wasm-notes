use std::fs;

use wasm_bindgen::prelude::*; //компиляция раст в жс
use gloo_storage::LocalStorage; //работа с хранилищем браузера
use web_sys::{window, Document, HtmlElement}; //вывод в браузере

//отображение заметки

#[wasm_bindgen]
pub fn display_note(note: &str) {
    // Получаем доступ к глобальному объекту window и документу
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    //ПРИВЯЗЫВАЕМСЯ К ЭЛЕМЕНТУ НА СТРАНИЦЕ

    // Создаем новый элемент div для заметки
    let note_div = document.create_element("div").expect("Failed to create note div");
    note_div.set_inner_html(note); // Устанавливаем текст заметки

    // Добавляем стили к элементу
    note_div.set_attribute("style", "background-color: #f0f0f0; padding: 10px; margin: 10px; border-radius: 5px; position: absolute; right: 10px; top: 50px;") 
        .expect("Failed to set styles");

    // Добавляем элемент в body документа
    document.body().expect("document should have a body").append_child(&note_div).expect("Failed to append note div");
}

//сохранение заметки

#[wasm_bindgen]
pub fn save_note(key: &str, note: &str) {
    // Сохраняем заметку в localStorage
    LocalStorage::set(key, note).expect("Unable to save note");
}

#[wasm_bindgen]
pub fn get_note(key: &str) -> Option<String> {
    // Получаем заметку из localStorage
    LocalStorage::get::<String>(key).ok()
}

#[wasm_bindgen]
pub fn remove_note(key: &str) {
    // Удаляем заметку из localStorage
    LocalStorage::delete(key);
}

/*
#[wasm_bindgen]
pub fn save_note(note: &str) {
    let file_path = "notes.md"; // Путь к файлу для сохранения заметки

    // Сохраняем заметку в файл
    fs::write(file_path, note).expect("Unable to write file");
}*/
