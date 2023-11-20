import { invoke } from "@tauri-apps/api";

function untickTodo(todos, id) {
  return todos.map((todo) => {
    if (todo.id === id) {
      todo.state = false;
    }
    return todo;
  });
}

function ArchivedTodo({ todo, todos, setTodos }) {
  return (
    <button
      className={`${
        todo.state
          ? "underline hover:bg-white hover:drop-shadow-md duration-100"
          : "line-through animate-fade"
      } px-1`}
      onClick={() => {
        setTodos(untickTodo(todos, todo.id));
        if (!todo.state) {
          setTimeout(async () => {
            await invoke("uncomplete_todo", { id: todo.id });
            invoke("get_archive").then((res) => {
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

export default ArchivedTodo;
