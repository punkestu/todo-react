import { invoke } from "@tauri-apps/api";

function tickTodo(todos, id) {
  return todos.map((todo) => {
    if (todo.id === id) {
      todo.state = true;
    }
    return todo;
  });
}

function Todo({ todo, todos, setTodos }) {
  return (
    <button
      className={`${
        todo.state
          ? "line-through animate-fade"
          : "underline hover:bg-white drop-shadow-md duration-200"
      } px-1`}
      onClick={() => {
        setTodos(tickTodo(todos, todo.id));
        if (todo.state) {
          setTimeout(async () => {
            await invoke("complete_todo", { id: todo.id });
            invoke("get_list").then((res) => {
              setTodos(res);
            });
          }, 500);
        }
      }}
    >
      {todo.state && <span className="me-1">✅️</span>}
      {todo.label}
    </button>
  );
}

export default Todo;
