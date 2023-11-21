import { invoke } from "@tauri-apps/api";

function tickTodo(todos, id) {
  return todos.map((todo) => {
    if (todo.id === id) {
      todo.state = true;
    }
    return todo;
  });
}

function RecentTodo({ todo, todos, setTodos }) {
  return (
    <button
      className={`${todo.state ? "line-through animate-fade" : "underline"}`}
      onClick={() => {
        setTodos(tickTodo(todos, todo.id));
        if (todo.state) {
          setTimeout(async () => {
            await invoke("complete_todo", { id: todo.id });
            invoke("get_recent")
              .then((res) => {
                if (res) {
                  setTodos([res]);
                } else {
                  setTodos([]);
                }
              })
              .catch((err) => {
                if(err === "todo not found"){
                  setTodos([]);
                }
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

export default RecentTodo;
