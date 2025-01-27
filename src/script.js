import init, { display_note } from './pkg/my_project.js';

async function run() {
    await init();

    // Пример вызова функции для отображения заметки
    display_note("Это моя заметка!");
}

run();

///shadow dom
// Создаем элемент
//const hostElement = document.createElement('div');
//document.body.appendChild(hostElement);

// Создаем теневое дерево
const shadowRoot = hostElement.attachShadow({ mode: 'open' });

// Добавляем содержимое в теневое дерево
shadowRoot.innerHTML = `
    <style>
        p {
            color: blue;
        }
    </style>
    <p>Это текст внутри Shadow DOM</p>
`;

///вставка элемента
// Получаем элемент-хост
const hostElement = document.querySelector('.host');

// Создаем теневое дерево
const shadowRoot = hostElement.attachShadow({ mode: 'open' });

// Счетчик для уникальности элементов
let counter = 0;

// Функция для добавления элемента в теневое дерево
function addElementToShadowDOM() {
    // Создаем новый элемент
    const newElement = document.createElement('div');
    newElement.textContent = `Элемент ${++counter} в Shadow DOM`;
    newElement.style.padding = '5px';
    newElement.style.border = '1px solid blue';
    newElement.style.margin = '5px';
    
    // Добавляем новый элемент в теневое дерево
    shadowRoot.appendChild(newElement);
}

// Обработчик событий нажатия клавиш
document.addEventListener('keydown', (event) => {
    // Проверяем сочетание клавиш (например, Ctrl + Shift + A)
    if (event.ctrlKey && event.key === 'Q') {
        addElementToShadowDOM();
        event.preventDefault(); // Предотвращаем действие по умолчанию
    }
});
